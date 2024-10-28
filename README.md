<div align="center">
  <a href= "https://cadastre.data.gouv.fr/static/images/logos/cadastre.data.gouv.fr.svg">
  <img src="https://cadastre.data.gouv.fr/static/images/logos/cadastre.data.gouv.fr.svg">
  </>
</div>

![ci](https://github.com/github/docs/actions/workflows/ci.yml/badge.svg)

# EDIGéO Exchange Format

The [`EDIGéO`](https://www.data.gouv.fr/s/resources/plan-cadastral-informatise/20170906-150737/standard_edigeo_2013.pdf) (_Electronic Data Interchange in the field of Geographic Information_) format was established
by the French standards association (AFNOR). EDIGéO is a standardized format commonly used in France for
the exchange of geographical information.

The top-level data structure for an EDIGéO dataset is the exchange. An exchange appears as a single .THF file.
This file does not hold the main data; instead it specifies which lots belong to the exchange. An exchange,
therefore, consists of one or more lots. A lot in EDIGéO is conceptually a dataset. Within a lot, all data is
self-contained. Therefore, opening an exchange file with multiple lots is conceptually identical to opening
several exchange files each having one lot.

An EDIGéO lot is described in several plain text files. These files are listed below:
`.GEN` - General Information
