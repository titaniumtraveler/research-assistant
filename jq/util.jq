def group_to_object(key; val):
  group_by(key)
  | [
    .[]
    | {
      "key": .[0].key,
      "value": val
    }
  ]
  | from_entries
  ;

def group_by_key(f):
  [
    .[]
    | to_entries[]
  ]
  | group_to_object(.key;[.[].value] | f)
  ;
