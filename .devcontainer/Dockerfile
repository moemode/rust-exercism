FROM mcr.microsoft.com/devcontainers/rust:bookworm

# Install exercism CLI
RUN curl -L https://github.com/exercism/cli/releases/download/v3.5.4/exercism-3.5.4-linux-x86_64.tar.gz -o exercism.tar.gz \
    && tar -xf exercism.tar.gz \
    && mv exercism /usr/local/bin/ \
    && chmod +x /usr/local/bin/exercism \
    && rm exercism.tar.gz