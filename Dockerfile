FROM ubuntu

RUN apt-get update -y && \
		apt-get install -y \
			clang \
			libssl-dev \
			curl \
			pkg-config && \
		curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN apt-get install make

RUN rm -rf /var/lib/{apt,dpkg,cache,log}/
ENV PATH "$PATH:/root/.cargo/bin"
WORKDIR /app


CMD [ "make","static" ]
