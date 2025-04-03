{{/*
Expand the name of the chart.
*/}}
{{- define "my-app.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "my-app.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "my-app.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "my-app.labels" -}}
helm.sh/chart: {{ include "my-app.chart" . }}
{{ include "my-app.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "my-app.selectorLabels" -}}
app.kubernetes.io/name: {{ include "my-app.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
PostgreSQL fullname
*/}}
{{- define "my-app.postgresql.fullname" -}}
{{- printf "%s-postgresql" .Release.Name }}
{{- end }}

{{/*
PostgreSQL host
*/}}
{{- define "my-app.postgresql.host" -}}
{{- include "my-app.postgresql.fullname" . }}
{{- end }}

{{/*
PostgreSQL port
*/}}
{{- define "my-app.postgresql.port" -}}
5432
{{- end }}

{{/*
PostgreSQL username
*/}}
{{- define "my-app.postgresql.username" -}}
{{- .Values.postgresql.auth.username }}
{{- end }}

{{/*
PostgreSQL password secret name
*/}}
{{- define "my-app.postgresql.secretName" -}}
{{- include "my-app.postgresql.fullname" . }}
{{- end }}

{{/*
PostgreSQL password secret key
*/}}
{{- define "my-app.postgresql.secretKey" -}}
password
{{- end }}

{{/*
PostgreSQL database name
*/}}
{{- define "my-app.postgresql.database" -}}
{{- .Values.postgresql.auth.database }}
{{- end }}
