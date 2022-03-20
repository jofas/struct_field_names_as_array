readme=$(cat README.md)

regex='<!-- toc -->([^#]|\n)*'

toc=$(echo "$readme" | ../github-markdown-toc/gh-md-toc -)

replace="<!-- toc -->\n$toc\n\n"

if [[ $readme =~ $regex ]]; then
  echo -e "${readme/${BASH_REMATCH[0]}/$replace}" > README.md
  echo "Successfully created toc"
else
  echo "Could not find place to insert toc into"
fi
