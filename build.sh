#!/bin/bash

set -e
[[ $DEBUG = [tT]rue ]] && set -x
LAST="${LAST:-HEAD^}"
export ORG=ustcmirror

is_modified() {
    [[ -n $(git diff "$LAST" HEAD -- "$1") ]]
}

build_image() {
    local image="$1"
    if [[ -x $image/build.sh ]]; then
        (cd "$image" && ./build.sh)
    else
        docker build -t "$ORG/$image" "$image"
        docker push "$ORG/$image"
    fi
}

docker login -u "$DOCKER_USER" -p "$DOCKER_PASS"

derived=(*sync 'test')
#########################################
### Images based on ustcmirror/base
#########################################
if is_modified "base"; then
    build_image base
    for image in "${derived[@]}"; do
        build_image "$image"
    done
else
    for image in "${derived[@]}"; do
        is_modified "$image" && build_image "$image"
    done
fi

############################################
### Images dosen't based on ustcmirror/base
############################################
others=(mongodb)
for image in "${others[@]}"; do
    is_modified "$image" && build_image "$image"
done

exit 0
