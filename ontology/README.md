
The `*.ttl` files listed here are used to generate rust parsers in the `ontology`
directory using the `rome` crate:

```rust
cargo run --example generate_code -- --mod ontology ../lv2/src/ontology ../lv2/ontology/*.ttl
```

This fork was used: `https://github.com/potocpav/rome`. Some details about
the local changes to the `ttl` files are in `ontology/README.md`.

## Tweaks

Some file parts were deleted or commented out becaus `rome` can't parse them.

A `subClassOf` attribute can't contain anonymous classes. This was found in:

 * `ui.ttl#201`
 * `lv2core.ttl#144`
 * `lv2core.ttl#169`

A `Property` without a `range` can't be loaded. This was the case in:

 * `foaf.ttl` - `Sha1`
 * `doap.ttl` - many properties
 * `ui.ttl` - `notifyType`

xsd datatypes are not loaded properly

 * `owl.ttl` - all `cardinality` properties

`Datatype`s are not considered `Class` if the `subClassOf` property is not present.
The `subClassOf` property was added to:

 * `xsd.ttl` - in multiple places
 * `lv2core.ttl` - `lv2:Symbol`

Multiple `rdfs:range` items:

 * `lv2core.ttl` - `lv2:symbol` had a `rdfs:range` of `rdf:PlainLiteral` removed
