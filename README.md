# plugin-generic-exporter

This [CNPG-i](http://github.com/cloudnative-pg/cnpg-i) plugin is a tool that
enhances the monitoring capabilities of CloudNativePG (CNPG) clusters by
seamlessly deploying the generic [SQL Prometheus exporter
container](https://github.com/justwatchcom/sql_exporter). 

## Installation

To use this plugin you need an installation of a plugin-enabled CloudNativePG
operator. The plugin need to be installed as sidecar of the operator. A simple
way to do that is to use the patch included in this repository like this:

```
kubectl patch deployment -n cnpg-system  cnpg-controller-manager --patch-file kubernetes/install-patch.json

kubectl rollout restart deployment -n cnpg-system  cnpg-controller-manager

kubectl rollout status deployment -n cnpg-system  cnpg-controller-manager
```

## Usage

To activate the plugin you need a `Cluster` definition referencing it and the
appropriate exported configuration. The following is a basic example of that:

```yaml
apiVersion: postgresql.cnpg.io/v1
kind: Cluster
metadata:
  name: cluster-example
spec:
  instances: 3

  plugins:
  - name: plugin-generic-exporter.leonardoce.io
    parameters:
      configMapName: sql-exporter-config

  storage:
    size: 1Gi
```

The `sql-exporter-config` configmap contains the exporter configuration. You can
find an example of that [in the source
code](./kubernetes/sql-exporter-config.yaml).

## Supported parameters

This plugin supports the following parameters:

* `configMapName` is the name of the ConfigMap where the exporter configuration
  is available. This parameter is required. The passed ConfigMap need to contain
  an entry called `config.yml` whose value is the configuration.

* `imageName` is the name of the image containing the generic SQL exporter. This
  parameters defaults to `ghcr.io/justwatchcom/sql_exporter:latest`.
