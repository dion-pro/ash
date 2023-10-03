# Ash - Comprehensive Readme 👀

Ash is a simple yet powerful shell, built in Rust with packages for many platforms.

Ash integrates well with Bash and fish, it also includes a custom shell script. 😄

Below is a comparison of Ashscript and Bash

## Comparison

Bash:
```bash
function main
    echo "Hello, World!"
end
```

```bash
echo "Bash version ${BASH_VERSION}..."
for i in {0..10..2}; do
  echo "Welcome $i times"
done
```

Ashscript:

```bash
function main {
    print("Hello, World!")
}
for 
```
```bash
echo "Bash version ${BASH_VERSION}..."
for i in (0..10..2) {
  echo "Welcome $i times"
}
```