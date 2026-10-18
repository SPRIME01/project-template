# Reference commands; Just is an optional convenience layer.
set dotenv-load := true

default:
    @just --list

setup:
    bun install

db-start:
    pg0 start --name project-template --port 5432

db-stop:
    pg0 stop --name project-template

migrate:
    bun run db:migrate

db-smoke:
    bun run db:smoke

db-ruvector:
    bun run db:ruvector

dev:
    bun run dev

api:
    bun run dev:api

web:
    bun run dev:web

check:
    bun run check

test:
    bun run test

build:
    bun run build

desktop-dev:
    bun run desktop:dev

desktop-build:
    bun run desktop:build

doctor:
    bun --version
    just --version
    pg0 --help

# Isolated optional renderer probes; not part of default check/build.
native-render-preact:
    bash experiments/native-react-blitz/scripts/probe.sh preact

native-render-astryx:
    bash experiments/native-react-blitz/scripts/probe.sh astryx

# Actual native Boa + Blitz execution tests.
native-render-headless-preact:
    bash experiments/native-react-blitz/scripts/probe.sh headless-preact

native-render-headless-astryx:
    bash experiments/native-react-blitz/scripts/probe.sh headless-astryx

native-render-headless-react:
    bash experiments/native-react-blitz/scripts/probe.sh headless-react

native-render-headless-react-input:
    bash experiments/native-react-blitz/scripts/probe.sh headless-react-input

native-render-headless-astryx-input:
    bash experiments/native-react-blitz/scripts/probe.sh headless-astryx-input
