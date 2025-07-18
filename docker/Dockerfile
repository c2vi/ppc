FROM debian

RUN apt update
RUN apt install -y openssl

ENV RUST_LOG=trace
RUN ln -s /modules/mize/mize /bin/mize

COPY ./modules /modules

CMD /modules/mize/bin/mize run
