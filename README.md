# Semitic Knowledge Graph

The Semitic Knowledge Graph (SKG) is a suite of tools, ontologies, server, and web
client that is designed around exploring texts and artificats from the ancient
Near East. The goal of the SKG is to place comparative semitic linguistics in
its historical and cultural contexts. 


## Parts of the Repo

### codex: The Rust Server 

Routes:

1. ./auth
1. / 
1. /query 
1. /update 
1. Do we need taxonomies and ontology routes?


### data: the raw data used for importing

### docs: documentation for different features 

### skg: The React Web Client 

### skg-scripts: scripts for parsing data 

Primarily, these are python scripts to parse data and pull in the data in a
pipeline or controled form. Many of the functions tested in the `skg-notes`
directory are incorporated into scripts here.

This directory also contains the subdirectory `skg-notes` which contains the
jupiter notebooks for exploring data.


### ontologies 

All of the ontologies have been serialized with the
[rdf-toolkit](https://github.com/edmcouncil/rdf-toolkit) with the options
`-ibn -i "  "` (inline blank nodes and an indentation of two spaces). This is
for human readable friendliness and standardization. 

The Bibframe, CIDOC-CRM, and the Gold ontologies are stored here for easy
reference and version assurance. The Gold ontology can be found in the way
back machine from archive.org. 

### taxonomies 



