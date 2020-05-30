# json2csv

Convert a flow of json objects from the standard input to a csv format on the standard output.

# Example

```bash
$ echo '{"a": 3, "b": 4, "c": 5} {"a": 6, "b": 7, "c": 8} {"b": 9, "c": 0}' | json2csv
a,b,c
3,4,5
6,7,8
,9,0
```

# Key selection

By default, `json2csv` uses the keys found in the first object encountered. But
keys can be specified by the `-k` option:

```bash
$ echo '{"a": 3, "b": 4, "c": 5} {"a": 6, "b": 7, "c": 8} {"b": 9, "c": 0}' | json2csv -k b,c
b,c
4,5
7,8
9,0
```

Keys can also be excluded:

```bash
$ echo '{"a": 3, "b": 4, "c": 5} {"a": 6, "b": 7, "c": 8} {"b": 9, "c": 0}' | json2csv -e a
b,c
4,5
7,8
9,0
```
