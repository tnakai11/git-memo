#!/bin/sh
# Push all memo references to the given remote (defaults to origin)
remote=${1:-origin}
exec git push "$remote" 'refs/memo/*:refs/memo/*'
