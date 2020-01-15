# Show TLS expiration

Show expiration date of TLS certificate.

## Prerequisite

Install openssl.

## Usage

    USAGE:
        tls_expires <host_name>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    ARGS:
        <host_name>    Host name to check the TLS expiration. Ex: www.ruimo.com

## How to check the expiration date.

The following shows an example to check the TLS expiration.

    #!/bin/bash
    EXP=$(./tls_expires www.ruimo.com)
    
    NOW=$(date +%s)
    EXP=$(date -d $EXP +%s)
    REST=$(($EXP-$NOW))
    THREE_WEEKS=$((60*60*24*7*3))
    
    if [ $REST -lt $THREE_WEEKS ]; then
      echo TLS will expire within 3 weeks!
      exit 1
    fi

## Binary

Linux(x86_64):
[0.1.0-SNAPSHOT](http://static.ruimo.com/release/tls_expires/0.1.0-SNAPSHOT/tls_expires)

