# ---- Build stage ----
FROM golang:1.23-alpine AS builder
WORKDIR /src
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -o /out/http-server .

# ---- Runtime stage ----
FROM alpine:3.20
RUN apk add --no-cache ca-certificates
COPY --from=builder /out/http-server /usr/local/bin/http-server
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/http-server"]