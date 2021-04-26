FROM google/cloud-sdk:alpine
COPY ./rocket-sqlite /usr/local/bin/
COPY ./entrypoint.sh /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]