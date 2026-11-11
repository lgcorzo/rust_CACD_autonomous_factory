> **Estado: Designed.** Este documento es un plan, no un registro de pruebas ejecutadas. El inventario de namespaces, endpoints y la interfaz de red de contenedor (CNI) activa es **TO CONFIRM**. En `deploy/` no hay objetos `Namespace`; los bindings de admisión seleccionan `development`, y la configuración refiere `factory-sandbox` y la etiqueta de namespace `openziti`. Los nombres y resultados de ejemplo siguientes no acreditan el estado actual. No se afirma una certificación externa.

# Plan de inspección de seguridad del entorno (Dark Gravity CA/CD V7.2)

**Documento:** Plan de Auditoría Operacional y Validación en Producción  
**Clasificación:** Seguridad Defensiva en Profundidad (Zero-Trust), Sandboxing y Gobernanza de Agentes IA  
**Estándares de Referencia:** EU AI Act, SOC 2 Type II, ISO/IEC 25059, OWASP Top 10 for LLMs (2025/2026), NIST AI RMF  
**Objetivo:** Proponer pasos para recopilar observaciones sobre los controles de seguridad. Este documento no registra resultados de ejecución.

---

## 1. Topología del Entorno Desplegado y Arquitectura de Control

En este documento, DAG significa grafo acíclico dirigido.

```
                      INTERFAZ DE CONTROL / OPERADOR HUMANO
                                            │
               ┌────────────────────────────┼────────────────────────────┐
               ▼                            ▼                            ▼
      GitLab / GitHub MRs             Hatchet Console             Grafana / Sentry
      (Revisión humana)        (Orquestador DAG)                  (Telemetría y Alertas)
               │                            │                            │
 ══════════════╪════════════════════════════╪════════════════════════════╪════════════════
 CLÚSTER KUBERNETES │                         │                            │
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

Los límites del diagrama corresponden al perfil configurado. El perfil más overhead suma `85Mi`/`450m` en límites; el builder por defecto usa `512Mi`/`500m`, en conflicto con el límite de admisión `30Mi`/`250m`. Estos son valores de configuración, no mediciones de uso.

---

## 2. Pre-requisitos y Herramientas del Auditor

Antes de ejecutar los ejemplos, confirma los nombres de los namespaces y servicios en el entorno objetivo. El repositorio no establece el inventario desplegado. En los archivos disponibles, `deploy/` no declara objetos `Namespace`; los bindings de admisión seleccionan `development`, mientras que la configuración menciona `factory-sandbox` y la etiqueta `openziti`.
Define `SANDBOX_NAMESPACE` con el nombre observado antes de ejecutar los comandos; su valor en un clúster activo es **TO CONFIRM**.
1. Acceso `kubectl` de solo lectura para los namespaces confirmados. Sus nombres están **TO CONFIRM**.
2. Binario `vault` CLI autenticado con rol de auditoría.
3. Herramientas CLI: `curl`, `jq`. Usa herramientas Cilium solo si el CNI activo es Cilium; el CNI está **TO CONFIRM**.
4. Binario de prueba sintética: `factory-cli` (ubicado en `crates/factory-cli`).

---

## 3. Matriz de Fases de Prueba en el Entorno Desplegado

---

### FASE 1: Auditoría de Aislamiento de Kernel y Sandboxing (VULN-03)

**Objetivo:** Demostrar que el código generado por IA nunca se ejecuta directamente sobre el kernel de los nodos anfitriones y que los límites de memoria son forzados por el kernel del sandbox.

#### Prueba 1.1: Verificación de RuntimeClass gVisor (`runsc`)
* **Comando:**
  ```bash
  kubectl get pods -n "$SANDBOX_NAMESPACE" -o jsonpath='{range .items[*]}{.metadata.name}{"\tRuntimeClass="}{.spec.runtimeClassName}{"\n"}{end}'
  ```
* **Criterio de Aprobación:** Todos los pods de ejecución deben retornar `RuntimeClass=gvisor`.
* **Resultado a registrar:** Un valor vacío o `runc` no satisface este criterio. Registra la salida observada; no infieras un resultado de certificación.

#### Prueba 1.2: Sonda de Intercepción de Syscalls de Kernel
* **Comando:**
  ```bash
  # Seleccionar un pod de sandbox activo y verificar intercepción de dmesg
  TARGET_POD=$(kubectl get pods -n "$SANDBOX_NAMESPACE" -l app=zeroclaw-worker -o jsonpath='{.items[0].metadata.name}')
  kubectl exec -n "$SANDBOX_NAMESPACE" "$TARGET_POD" -- dmesg
  ```
* **Criterio de Aprobación:** El comando debe fallar con `dmesg: klogctl: Operation not permitted` (bloqueado en espacio de usuario por el Sentry de gVisor).

#### Prueba 1.3: Verificación de Sistema de Archivos Inmutable (Read-Only Root)
* **Comando:**
  ```bash
  kubectl exec -n "$SANDBOX_NAMESPACE" "$TARGET_POD" -- touch /test_file.txt
  ```
* **Criterio de Aprobación:** Debe devolver `touch: cannot touch '/test_file.txt': Read-only file system`. Únicamente se permite escritura en `/tmp` volátil tipo `emptyDir`.

#### Prueba 1.4: Comparación de límites declarados y política
El manifiesto de admisión limita la carga declarada a `30Mi`/`250m`; el builder usa por defecto `512Mi`/`500m`. Registra los valores del Pod y el resultado de admisión. La causa del conflicto debe resolverse antes de interpretar una prueba de OOM. El comportamiento de admisión y el uso real son **TO CONFIRM**.

---

### FASE 2: Auditoría de Aislamiento de Red y Restricción Egress

**Objetivo:** Comprobar que ningún agente puede exfiltrar código, credenciales o telemetría hacia Internet o hacia componentes de la intranet corporativa no autorizados.

#### Prueba 2.1: Sonda de Egress hacia Internet Público
* **Comando:**
  ```bash
  kubectl exec -n "$SANDBOX_NAMESPACE" "$TARGET_POD" -- curl -I -m 5 https://api.openai.com
  kubectl exec -n "$SANDBOX_NAMESPACE" "$TARGET_POD" -- curl -I -m 5 https://1.1.1.1
  ```
* **Resultado a registrar:** Anota la salida de cada intento. El CNI activo y el comportamiento de egreso son **TO CONFIRM**.

#### Prueba 2.2: Inspección de políticas de red
* **Comando:**
  ```bash
  kubectl get networkpolicies -n "$SANDBOX_NAMESPACE"
  kubectl get ciliumnetworkpolicies -n "$SANDBOX_NAMESPACE"
  ```
* La configuración incluye objetos Kubernetes `NetworkPolicy` y Cilium `CiliumNetworkPolicy`; el CNI instalado y la aplicación efectiva son **TO CONFIRM**.
* El perfil de sandbox separado permite tráfico al namespace con etiqueta `openziti` en TCP `8443` y `10080`. El endpoint activo es **TO CONFIRM**.

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

**Resultados de ejecución: TO CONFIRM.** No se incluye aquí una salida de ejecución registrada. Complete una fila de la matriz siguiente solo después de ejecutar la prueba y guardar su evidencia.

---

## 5. Matriz de resultados y plantilla de informe

| Control propuesto | Resultado | Evidencia y alcance |
| :--- | :--- | :--- |
| RuntimeClass gVisor (`runsc`) | **TO CONFIRM** | Registrar el valor observado y el namespace objetivo. |
| Límites de recursos | **TO CONFIRM** | Registrar requests, limits, overhead y resultado de admisión. |
| Política de red | **TO CONFIRM** | Registrar el objeto, CNI activo y resultado de la prueba. |
| Secretos Vault | **TO CONFIRM** | Registrar TTL y alcance observados. |
| Identidad Ed25519 | **TO CONFIRM** | Registrar qué payload se inspeccionó y qué firma se comprobó. |
| Análisis de código | **TO CONFIRM** | Registrar herramienta, alcance y salida. |
| Circuit breaker | **TO CONFIRM** | Registrar el caso ejecutado y su salida. |
| FinOps | **TO CONFIRM** | Registrar periodo, umbral y resultado observado. |
| Revisión humana | **TO CONFIRM** | Registrar la configuración del proveedor y su alcance. |

**Dictamen final:** **TO CONFIRM**. Este plan no emite una conclusión de certificación.
**Firma del Auditor / CISO:** _____________________________  
**Fecha de Inspección:** ___________________________________
