export default JSON.stringify(
{
	"$schema": "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
	"name": "roost",
	"patterns": [
		{
			"include": "#comments"
		},
		{
			"include": "#keywords"
		},
		{
			"include": "#constants"
		},
		{
			"include": "#strings"
		},
		{
			"include": "#functions"
		},
		{
			"include": "#variables"
		}
	],
	"repository": {
		"comments": {
			"patterns": [
				{
					"name": "comment.line.double-slash.roost",
					"match": "//.*"
				},
				{
					"name": "comment.block.roost",
					"begin": "/\\|",
					"end": "\\|/"
				}
			]
		},
		"keywords": {
			"patterns": [
				{
					"name": "keyword.control.roost",
					"match": "\\b(if|else|loop|while|for|return|break|continue|in)\\b"
				},
				{
					"name": "storage.type.roost",
					"match": "\\b(var|fun)\\b"
				}
			]
		},
		"constants": {
			"patterns": [
				{
					"name": "constant.language.roost",
					"match": "\\b(true|false|null)\\b"
				},
				{
					"name": "constant.numeric.roost",
					"match": "\\b\\d[\\d_]*(\\.\\d[\\d_]*)?\\b"
				},
				{
					"name": "variable.language.roost",
					"match": "\\banswer\\b"
				}
			]
		},
		"strings": {
			"patterns": [
				{
					"name": "string.quoted.double.roost",
					"begin": "\"",
					"end": "\"",
					"patterns": [
						{
							"name": "constant.character.escape.roost",
							"match": "\\\\(n|r|t)"
						}
					]
				},
				{
					"name": "string.quoted.single.roost",
					"begin": "'",
					"end": "'",
					"patterns": [
						{
							"name": "constant.character.escape.roost",
							"match": "\\\\(n|r|t)"
						}
					]
				}
			]
		},
		"functions": {
			"patterns": [
				{
					"name": "support.function.roost",
					"match": "\\b(printl?|typeOf)\\b"
				},
				{
					"name": "meta.function.definition.roost",
					"begin": "\\b(fun)\\b\\s+([A-Za-z_]\\w*)\\s*\\(",
					"beginCaptures": {
						"1": {
							"name": "keyword.other.fun.roost storage.type.roost"
						},
						"2": {
							"name": "entity.name.function.roost"
						}
					},
					"end": "\\)",
					"patterns": [
						{
							"include": "#comments"
						},
						{
							"include": "#keywords"
						},
						{
							"include": "#constants"
						},
						{
							"include": "#strings"
						},
						{
							"include": "#functions"
						},
						{
							"include": "#variables"
						}
					]
				},
				{
					"name": "meta.function.call.roost",
					"begin": "\\b([A-Za-z_]\\w*)\\(",
					"beginCaptures": {
						"1": {
							"name": "entity.name.function.roost"
						}
					},
					"end": "\\)",
					"patterns": [
						{
							"include": "#comments"
						},
						{
							"include": "#keywords"
						},
						{
							"include": "#constants"
						},
						{
							"include": "#strings"
						},
						{
							"include": "#functions"
						},
						{
							"include": "#variables"
						}
					]
				}
			]
		},
		"variables": {
			"patterns": [
				{
					"name": "variable.other.roost",
					"match": "\\b(?<!(?<!\\.)\\.)[A-Za-z_]\\w*\\b"
				}
			]
		}
	},
	"scopeName": "source.roost"
}
)
