import "jq/util" as util;

def map_with_type(n;str;obj;arr):
  if   type == "null"   then n
  elif type == "string" then str
  elif type == "object" then obj
  elif type == "array"  then arr
  else error
  end
  ;

[ .[] | to_entries[] ]
| util::group_to_object(.key;[.[].value[]])
| [
  .portals[]
  # | .otherworldid
  # | type
  # | map_with_type
  #   ( empty
  #   ; error
  #   ; error
  #   ; .[]
  # )
]
| unique
# | util::group_by_key(unique)
# | keys
# | sort_by(. | (. | ascii_downcase), .)
