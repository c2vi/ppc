FROM debian

RUN apt update
RUN apt install -y openssl

ENV MIZE_CONFIG=module_dir.ppc=/modules/ppc:load_modules=ppc:store_path=/mize
ENV RUST_LOG=trace
RUN ln -s /modules/mize/mize /bin/mize

COPY ./modules /modules

CMD /modules/mize/mize run
