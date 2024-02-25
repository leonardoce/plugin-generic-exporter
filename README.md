# plugin-generic-exporter

This [CNPG-i](http://github.com/cloudnative-pg/cnpg-i) plugin is a tool that
enhances the monitoring capabilities of CloudNativePG (CNPG) clusters by
seamlessly deploying the generic [SQL Prometheus exporter
container](https://github.com/justwatchcom/sql_exporter). 

To use this plugin you need an installation of a plugin-enabled CloudNativePG
operator, and a `Cluster` definition referencing the plugin and the appropriate
exported configuration:

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
