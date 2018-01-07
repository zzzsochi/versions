FROM python:3-alpine

COPY versions.py /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/versions.py"]
