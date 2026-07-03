FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie
RUN apt-get update
RUN apt-get install -y iverilog
RUN apt-get install -y yosys

RUN apt-get install -y python3-venv
RUN apt-get install -y python3-pip
ENV VIRTUAL_ENV=/opt/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"
COPY requirements.txt .
RUN pip install -r requirements.txt