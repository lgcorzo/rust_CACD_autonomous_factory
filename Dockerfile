# https://docs.docker.com/engine/reference/builder/

FROM python:3.12
COPY dist/*.whl .
RUN pip install *.whl
ARG ENTRYPOINT_MODE=mcp
ENV ENTRYPOINT_MODE=${ENTRYPOINT_MODE}
CMD ["sh", "-c", "if [ \"$ENTRYPOINT_MODE\" = 'mcp' ]; then python -m autogen_team.application.mcp.mcp_server; else python -m autogen_team.infrastructure.messaging.kafka_app; fi"]
