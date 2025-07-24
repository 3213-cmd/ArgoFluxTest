# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the `sampleserver` component of the ArgoFlux project - a minimal Rust web application built with Axum that displays Kubernetes environment information. It's designed for deployment with ArgoCD or Flux.

## Development Commands

- `cargo run` - Run the development server (defaults to port 3000)
- `cargo build --release` - Build optimized binary
- `cargo check` - Check code without building
- `docker build -t sampleserver .` - Build Docker image
- `docker run -p 3000:3000 sampleserver` - Run containerized application

## Environment Variables

The application reads these Kubernetes environment variables:
- `NAMESPACE` - Kubernetes namespace (defaults to "default")
- `POD_NAME` - Pod name (defaults to "unknown-pod")  
- `NODE_NAME` - Node name (defaults to "unknown-node")
- `SERVICE_ACCOUNT` - Service account (defaults to "default")
- `PORT` - Server port (defaults to "3000")
- `APP_*` - Any variables starting with APP_ are displayed as custom variables

## Architecture

- **Web Framework**: Axum for HTTP handling
- **Templating**: Askama for HTML templating with embedded CSS
- **Structure**: Single-binary application with embedded templates
- **Endpoints**:
  - `/` - Main page displaying environment info
  - `/health` - Health check endpoint
  - `/mock_tests` - Returns configurable number from config.toml
  - `/?debug=1` - Debug mode with additional environment variables

## Repository Structure

- `src/main.rs` - Main application server code
- `templates/index.html` - HTML template with embedded CSS styling
- `config.toml` - Configuration file for mock_tests endpoint
- `Cargo.toml` - Rust dependencies and project configuration
- `Dockerfile` - Multi-stage build for containerization