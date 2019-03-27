FROM debian:latest

COPY nervmichnich.patch /nervmichnich.patch

RUN apt update && apt upgrade && apt-get install socat ccache cmake make g++-multilib gdb \
        pkg-config coreutils python3-pexpect python-pexpect manpages-dev git \
        ninja-build capnproto libcapnp-dev -y &&\
    git clone https://github.com/mozilla/rr.git &&\
    cd rr && patch -p1 < /nervmichnich.patch && cd .. &&\
    mkdir obj && cd obj &&\
    cmake ../rr &&\
    make -j8 &&\
    make install &&\
    cd .. && rm -r obj && rm -r rr

# setup user
ARG uid
ARG gid

RUN addgroup --gid ${gid} pwndock && adduser --uid ${uid} --gid ${gid} --disabled-password pwndock
USER pwndock
WORKDIR /home/pwndock


# copy scripts
COPY run.sh /home/pwndock/run.sh
COPY debug.sh /home/pwndock/debug.sh
COPY execute.sh /home/pwndock/execute.sh
CMD ["/home/pwndock/run.sh"]
