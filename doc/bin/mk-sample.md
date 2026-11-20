pandoc list-table-sample.md \
    --lua-filter bin/list-table.lua\
    --to=json \
 | jq .
# | pandoc -s \
#   --from json \
#   --to markdown+pipe_tables 

