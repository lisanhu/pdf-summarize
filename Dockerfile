FROM rust:latest

RUN apt-get update && apt-get install -y python3 python3-pip
RUN pip3 install --no-cache-dir --upgrade pip setuptools wheel


COPY . .

WORKDIR /pdf-summarize-bin

RUN cargo build --release

WORKDIR /pdf-summarize-website

RUN pip3 install --no-cache-dir -r requirements.txt

EXPOSE 5000

ENTRYPOINT gunicorn --timeout 900 -b 0.0.0.0:5000 server:app
