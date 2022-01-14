FROM envoyproxy/envoy:v1.19.0
ADD envoy.yaml /srv/envoy.yaml
CMD envoy -c /srv/envoy.yaml

