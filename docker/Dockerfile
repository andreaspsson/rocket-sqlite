FROM google/cloud-sdk:alpine
COPY ./rocket-sqlite /usr/local/bin/
COPY ./entrypoint.sh /usr/local/bin/
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]