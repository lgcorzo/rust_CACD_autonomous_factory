# Plan de Pruebas de Auditoría de Seguridad para el Entorno Desplegado (Dark Gravity CA/CD V7.2)

**Documento:** Plan de Auditoría Operacional y Validación en Producción  
**Clasificación:** Seguridad Defensiva en Profundidad (Zero-Trust), Sandboxing y Gobernanza de Agentes IA  
**Estándares de Referencia:** EU AI Act, SOC 2 Type II, ISO/IEC 25059, OWASP Top 10 for LLMs (2025/2026), NIST AI RMF  
**Objetivo:** Guía técnica y procedimental paso a paso con comandos ejecutables para verificar empíricamente sobre el clúster en vivo que la factoría cumple con todos los controles de seguridad sin confiar en declaraciones estáticas.

---

## 1. Topología del Entorno Desplegado y Arquitectura de Control

```
                      INTERFAZ DE CONTROL / OPERADOR HUMANO (HITL)
                                            │
               ┌────────────────────────────┼────────────────────────────┐
               ▼                            ▼                            ▼
      GitLab / GitHub MRs             Hatchet Console             Grafana / Sentry
      (Merge Protegido HITL)      (Orquestador de DAGs)         (Telemetría y Alertas)
               │                            │                            │
 ══════════════╪════════════════════════════╪════════════════════════════╪════════════════
 CLÚSTER K8S   │                            │                            │
               ▼                            ▼                            ▼
   ┌──────────────────────┐     ┌──────────────────────┐     ┌──────────────────────┐
   │ LiteLLM Proxy Gate   │◄────┤ Worker Pod (ZeroClaw)│────►│ HashiCorp Vault      │
   │ - Modelo Juez SAST   │     │ - Runtime: gVisor    │     │ - JIT TTL 5 min      │
   │ - FinOps vTags       │     │ - RAM <= 30 MiB      │     │ - Paths aislados     │
   │ - HardStop al 90%    │     │ - Sidecar Ziti <=20M │     │ - Zeroize en RAM     │
   └──────────────────────┘     └──────────────────────┘     └──────────────────────┘
               │                            │                            │
               │                            ▼                            │
               │               ┌──────────────────────┐                  │
               └──────────────►│ Kafka Message Bus    │◄─────────────────┘
                               │ - Topic mission-events│
                               │ - Firmas Ed25519 JWS │
                               └──────────────────────┘
```

---

## 2. Pre-requisitos y Herramientas del Auditor

Para ejecutar esta auditoría sobre el entorno en producción, el auditor debe contar con:
1. Acceso `kubectl` con rol de solo lectura o inspector en los namespaces:
   - `factory-sandbox` (donde corren los agentes)
   - `llm-apps` (LiteLLM, R2R GraphRAG)
   - `orchestrators` (Hatchet)
   - `vault` (HashiCorp Vault)
2. Binario `vault` CLI autenticado con rol de auditoría.
3. Herramientas CLI: `curl`, `jq`, `cilium` (o `kubectl-cilium`).
4. Binario de prueba sintética: `factory-cli` (ubicado en `crates/factory-cli`).

---

## 3. Matriz de Fases de Prueba en el Entorno Desplegado

---

### FASE 1: Auditoría de Aislamiento de Kernel y Sandboxing (VULN-03)

**Objetivo:** Demostrar que el código generado por IA nunca se ejecuta directamente sobre el kernel de los nodos anfitriones y que los límites de memoria son forzados por el kernel del sandbox.

#### Prueba 1.1: Verificación de RuntimeClass gVisor (`runsc`)
* **Comando:**
  ```bash
  kubectl get pods -n factory-sandbox -o jsonpath='{range .items[*]}{.metadata.name}{"\tRuntimeClass="}{.spec.runtimeClassName}{"\n"}{end}'
  ```
* **Criterio de Aprobación:** Todos los pods de ejecución deben retornar `RuntimeClass=gvisor`.
* **Resultado Inaceptable:** Cualquier valor vacío o `runc` invalida inmediatamente la certificación.

#### Prueba 1.2: Sonda de Intercepción de Syscalls de Kernel
* **Comando:**
  ```bash
  # Seleccionar un pod de sandbox activo y verificar intercepción de dmesg
  TARGET_POD=$(kubectl get pods -n factory-sandbox -l app=zeroclaw-worker -o jsonpath='{.items[0].metadata.name}')
  kubectl exec -n factory-sandbox "$TARGET_POD" -- dmesg
  ```
* **Criterio de Aprobación:** El comando debe fallar con `dmesg: klogctl: Operation not permitted` (bloqueado en espacio de usuario por el Sentry de gVisor).

#### Prueba 1.3: Verificación de Sistema de Archivos Inmutable (Read-Only Root)
* **Comando:**
  ```bash
  kubectl exec -n factory-sandbox "$TARGET_POD" -- touch /test_file.txt
  ```
* **Criterio de Aprobación:** Debe devolver `touch: cannot touch '/test_file.txt': Read-only file system`. Únicamente se permite escritura en `/tmp` volátil tipo `emptyDir`.

#### Prueba 1.4: Contención de Memoria (Clamping OOM-Kill $\le 30\text{ MiB}$)
* **Procedimiento:** Ejecutar un script de alocación de $35\text{ MiB}$ dentro del contenedor de la aplicación:
  ```bash
  kubectl exec -n factory-sandbox "$TARGET_POD" -- python3 -c 'bytearray(35 * 1024 * 1024)'
  ```
* **Criterio de Aprobación:** El proceso debe ser terminado inmediatamente con código `137 (OOMKilled)`. El nodo host y los pods vecinos no deben experimentar incremento en presión de memoria (`MemoryPressure`).

---

### FASE 2: Auditoría de Aislamiento de Red y Restricción Egress

**Objetivo:** Comprobar que ningún agente puede exfiltrar código, credenciales o telemetría hacia Internet o hacia componentes de la intranet corporativa no autorizados.

#### Prueba 2.1: Sonda de Egress hacia Internet Público
* **Comando:**
  ```bash
  kubectl exec -n factory-sandbox "$TARGET_POD" -- curl -I -m 5 https://api.openai.com
  kubectl exec -n factory-sandbox "$TARGET_POD" -- curl -I -m 5 https://1.1.1.1
  ```
* **Criterio de Aprobación:** Todos los intentos deben terminar en `Connection timed out` (cero paquetes transmitidos fuera del clúster).

#### Prueba 2.2: Inspección de Políticas de Red (NetworkPolicy / Cilium)
* **Comando:**
  ```bash
  kubectl get networkpolicies -n factory-sandbox
  kubectl describe networkpolicy deny-all-egress -n factory-sandbox
  ```
* **Criterio de Aprobación:** La política debe especificar `Policy Types: Egress` con regla por defecto `Deny` y allow-list restrictivo limitado exclusivamente a:
  - OpenZiti Router (`10.96.0.10:1280` mTLS)
  - Kafka Cluster (`10.96.0.12:9092` SASL/Plain)

---

### FASE 3: Auditoría de Identidad Criptográfica NHI y HashiCorp Vault (VULN-04)

**Objetivo:** Verificar que las credenciales de los agentes son efímeras (5 minutos), no renovables, y que las acciones quedan registradas con firmas asimétricas Ed25519 inmutables.

#### Prueba 3.1: Auditoría de TTL y No Renovabilidad en Vault
* **Procedimiento:**
  1. Identificar el token JIT emitido en la última misión.
  2. Consultar sus propiedades en el API de Vault:
  ```bash
  vault token lookup <token_emitted>
  ```
* **Criterio de Aprobación:**
  - `renewable: false`
  - `creation_ttl <= 300` (5 minutos)
  - `meta.agent_id`: Debe identificar al agente (`zeroclaw-worker`).
  - `meta.audience`: Debe coincidir estrictamente con el repositorio de la misión.

#### Prueba 3.2: Expiración Automática de Tokens
* **Procedimiento:** Esperar 301 segundos desde la emisión del token y repetir la consulta:
  ```bash
  vault token lookup <token_emitted>
  ```
* **Criterio de Aprobación:** Vault debe responder con `Error looking up token: bad token` confirmando su purga absoluta.

#### Prueba 3.3: Prueba de Aislamiento de Rutas (*Cross-Tenant Isolation*)
* **Procedimiento:** Usando el token de un agente asignado a `repo-A`, intentar leer secretos de `repo-B`:
  ```bash
  VAULT_TOKEN="<token-repo-A>" vault kv get secret/data/repos/repo-B/access_token
  ```
* **Criterio de Aprobación:** Vault debe responder con código HTTP `403 (Permission Denied)`.

#### Prueba 3.4: Verificación de Causalidad y Firma W3C en Kafka
* **Comando:**
  ```bash
  # Leer el último evento publicado en el tópico mission-events
  kcat -b kafka.orchestrators.svc.cluster.local:9092 -t mission-events -C -e -o -1 -q | jq .
  ```
* **Criterio de Aprobación:** El payload JSON debe contener un objeto `verifiable_credential` con:
  - `type`: `["VerifiableCredential", "AgentTaskCredential"]`
  - `proof.type`: `Ed25519Signature2020`
  - `proof.jws`: Firma válida generada con la clave pública registrada en Vault.

---

### FASE 4: Auditoría de Disyuntor Adaptativo y Puerta SAST Heterogénea (VULN-01 & VULN-02)

**Objetivo:** Verificar que parches peligrosos no puedan llegar a Git y que los bucles infinitos de remediación sean abortados de inmediato.

#### Prueba 4.1: Validación de Independencia del Juez SAST en LiteLLM
* **Comando:**
  ```bash
  curl -s http://litellm.llm-apps.svc.cluster.local:4000/model/info | jq '.data[] | {model_name: .model_name, litellm_provider: .litellm_params.model}'
  ```
* **Criterio de Aprobación:** Se debe constatar que:
  - `code_generator` usa modelos locales o soberanos (ej. DeepSeek-Coder, Qwen 2.5).
  - `security_review` (SAST) usa **imperativamente** una familia de modelo frontera independiente (ej. Anthropic Claude 3.5 Sonnet / Azure OpenAI).

#### Prueba 4.2: Sonda Adversarial de Rechazo SAST ($\ge 8.0/10.0$)
* **Procedimiento:** Ejecutar una llamada directa a la herramienta MCP `security_review` inyectando código con comandos peligrosos:
  ```bash
  curl -s -X POST http://factory-mcp.orchestrators.svc.cluster.local:8080/call_tool \
       -H "Content-Type: application/json" \
       -d '{"name": "security_review", "arguments": {"diff": "+ let token = \"sk-live-1234567890\"; eval(user_param);"}}' | jq .
  ```
* **Criterio de Aprobación:**
  - `score`: Debe ser $< 8.0/10.0$ (típicamente $3.0$ o $0.0$).
  - `status`: `"rejected"`.
  - `findings`: Debe enumerar `Potential hardcoded secret` y `Potential Code Execution (eval)`.

#### Prueba 4.3: Sonda de Detección de Deadlock de Diff Hashing (Intento 2)
* **Procedimiento:** Simular en el orquestador dos intentos consecutivos con el mismo diff.
* **Criterio de Aprobación:** En el **intento 2**, el `CircuitBreakerGuard` debe calcular el mismo hash SHA-256 y disparar inmediatamente el estado `AgentStuck (Deadlock detected on attempt 2)`, congelando el DAG y revocando el token de Vault.

---

### FASE 5: Auditoría de Gobernanza FinOps y Corte Duro al 90% (VULN-05)

**Objetivo:** Comprobar que todos los costos de inferencia son trazables por microservicio y que se aplica el corte duro de emergencia al 90% del presupuesto diario.

#### Prueba 5.1: Auditoría de Inyección de Cabeceras vTags
* **Comando:**
  ```bash
  curl -s http://litellm.llm-apps.svc.cluster.local:4000/spend/logs | jq '.[0].metadata'
  ```
* **Criterio de Aprobación:** Cada llamada debe incluir obligatoriamente:
  ```json
  {
    "x-vtags-team": "dark-gravity-ops",
    "x-vtags-epic": "E6.3",
    "x-vtags-microservice": "factory-application",
    "x-vtags-cost-center": "eu-rd-grants"
  }
  ```

#### Prueba 5.2: Prueba de Disparo de HardStop al 90% ($45.00 de $50.00)
* **Procedimiento:** Simular mediante script de prueba un gasto acumulado de $\$45.00$ en la sesión.
* **Criterio de Aprobación:**
  1. Las siguientes solicitudes de inferencia son rechazadas con código HTTP `429 (HardStop Budget Exceeded)`.
  2. Se despacha el evento de emergencia al tópico Kafka `factory-alerts`.
  3. El workflow en Hatchet se detiene en seco.

---

### FASE 6: Auditoría de Gobernanza Humana (HITL)

**Objetivo:** Comprobar que ningún agente de IA tiene privilegios para mezclar código en `main` o desplegar a producción sin confirmación humana.

#### Prueba 6.1: Verificación de Regla de Fusión Manual
* **Procedimiento:**
  - Inspeccionar la configuración de ramas protegidas en GitLab / GitHub:
    - Rama `main`: Requiere aprobación obligatoria de Code Owners humanos.
    - Cuenta bot de Dark Gravity: No tiene permisos de *Bypass Protections*.
* **Criterio de Aprobación:** Cualquier intento de ejecución de merge automático por parte del bot resulta en error `403 Forbidden: Protected branch requires manual human review`.

---

## 4. Ejecución de la Sonda Integral Unificada (*Canary Mission*)

El clúster dispone de una misión canario automatizada que ejecuta todas las fases anteriores de forma encadenada:

```bash
cargo run --bin run_functional_suite
```

**Salida esperada en clúster conforme:**
```
============================================================
   DARK GRAVITY AUTONOMOUS FACTORY - FUNCTIONAL TEST SUITE  
============================================================

[Phase 1] Pre-flight Infrastructure & Resource Clamping Probe...
  [✓] Hatchet Orchestration Service: OK (200)
  [✓] R2R GraphRAG Semantic Memory: OK (200)
  [✓] LiteLLM Gateway & mTLS: OK (200)
  [✓] Kafka Event Bus Connectivity: OK (200)

[Phase 2] Zero-Trust Micro-Sandboxing Validation...
  [✓] gVisor Kernel Isolation Active: OK
  [✓] Application Container RAM Clamping (<= 30 MiB): ENFORCED
  [✓] OpenZiti Sidecar RAM Clamping (<= 20 MiB): ENFORCED
  [✓] Network Egress Drop Policy: ENFORCED

[Phase 3] Cryptographic Governance & Circuit Breaker...
  [✓] HashiCorp Vault 5-min TTL & Path Isolation: COMPLIANT
  [✓] Heterogeneous SAST Judge Threshold (>= 8.0/10.0): ENFORCED
  [✓] Deadlock Detection on Duplicate Diffs: VERIFIED
  [✓] FinOps 90% HardStop Emergency Cutoff: VERIFIED

STATUS: 100% PASS - CLUSTER CERTIFIED FOR PRODUCTION
```

---

## 5. Acta de Conformidad y Plantilla de Dictamen

| Control Auditado | Veredicto | Observaciones / Evidencia |
| :--- | :---: | :--- |
| **1. Kernel gVisor (`runsc`)** | `PASS / FAIL` | `kubectl get pod -o jsonpath='{.spec.runtimeClassName}'` = `gvisor` |
| **2. Clamping RAM $\le 30\text{ MiB}$** | `PASS / FAIL` | OOM-Kill verificado en espacio de usuario sin degradación de nodo |
| **3. Aislamiento de Red (Egress)** | `PASS / FAIL` | Paquetes salientes a Internet descartados por Cilium / NetPol |
| **4. Vault JIT TTL 5 minutos** | `PASS / FAIL` | Token expira a los 300s, renovación rechazada |
| **5. Firmas NHI Ed25519** | `PASS / FAIL` | Verificable Credential W3C validado en Kafka |
| **6. SAST Juez Heterogéneo** | `PASS / FAIL` | Modelo independiente Claude 3.5 Sonnet evaluó con umbral $\ge 8.0$ |
| **7. Disyuntor Deadlock Trip** | `PASS / FAIL` | Misión detenida en intento 2 ante diff hash duplicado |
| **8. FinOps HardStop al 90%** | `PASS / FAIL` | Inferencia abortada al alcanzar $\$45.00$ de $\$50.00$ |
| **9. Gobernanza HITL** | `PASS / FAIL` | Auto-merge prohibido; revisión humana manual obligatoria |

**Dictamen Final:** `APROBADO PARA OPERACIÓN EN PRODUCCIÓN SOBERANA` / `NO CONFORME`  
**Firma del Auditor / CISO:** _____________________________  
**Fecha de Inspección:** ___________________________________
