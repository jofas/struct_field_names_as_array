README="# struct-field-names-as-array\n"

while read -r line; do
  if [[ $line =~ ^//! ]]; then
    README="$README\n${line:4}"
  fi
done < src/lib.rs

printf "$README"
