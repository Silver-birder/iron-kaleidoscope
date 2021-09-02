#!/bin/sh

# Call this script from the root of the repo
for i in `cat chapters/chapters.list`;
do
    scripts/generate-chapter.sh $i
done
