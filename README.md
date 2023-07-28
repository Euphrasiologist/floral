# floral

Did you ever want to have the floral formulae of all flowering plant families right at your fingertips on the command line? No? Well I did, hence this tool.

It's very simple with only a few commands. It works by parsing a database (in `./assets/formula.csv`) into a pretty strongly typed struct `Formula`.

## Installation

It's the usual Rust story:

```
git clone https://github.com/Euphrasiologist/floral
cd floral
cargo install --release
floral -h
```

It's not yet mature enough to put on crates.io/here as binaries, but I'll get to that in due course.

## Usage

For example:

`floral proteaceae` will print all floral formulae associated with the family Proteaceae.

`floral -o proteales` will print all floral formulae associated with the order Proteales.

`floral -e proteaceae` will hopefully give a reasonably good explanation of the floral formula associated with the Proteales.

`floral -a` will print all floral formulae in the database.

And an example output here (Orchidaceae):

```
# run with 'floral orchidaceae'

Asparagales -> Orchidaceae -> Bisexual
X(↑),T5+1,A1-2,̅G3;capsule
          ╰────╯
```

And general usage:

```
floral v0.1

USAGE:
  floral [FLAGS] <STRING>

FLAGS:
  -h, --help            Prints help information
  -a, --all             Print all family information
  -e, --explain         Explain the floral formula
  -v, --version         Print version information only
  -o, --order           Search plant orders, not families

ARGS:
  <STRING>              Flowering plant family/order (with -o) name
```

## Data disclaimer

I've poached these floral formulae from the internet, Plant Systematics, A Phylogenetic Approach (Judd et al., 4th Ed 2016), and Floral Diagrams (Ronse De Crane, 2010). Oftentimes they are a combination of all the things I have found.

If you would like to add floral formulae, or amend the ones you see here, please put in a PR/issue and we can sort them out.

## TODO's / unimplemented 

- Major differences within a whorl (i.e. K2:2) where there are four sepals of two distinct types, is not yet implemented
- Multiple possiblities within a floral part not implemented, (e.g. A2 or A5-10). Note that tepals OR petals + sepals is implemented (e.g. T5-10 [or K5,C5])