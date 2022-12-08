#! /bin/sh

ROOT_CMD=""

if which "doas" > /dev/null 2>&1
then
    ROOT_CMD="doas"
else
    if which "sudo" > /dev/null 2>&1
    then
        ROOT_CMD="sudo";
    else
        echo '`sudo` or `doas` needs to be installed';
        exit 1;
    fi
fi

echo Rgx install script
echo 

# Compile rgx
echo 'Step 1: Compile rgx'
cargo build --release 
if [ $? -ne 0 ]; then exit 1; fi

# Move rgx to /usr/bin
echo 'Step 2: Move rgx into /usr/bin'
$ROOT_CMD cp -f "target/release/rgx" "/usr/bin/rgx"
if [ $? -ne 0 ]; then exit 1; fi
