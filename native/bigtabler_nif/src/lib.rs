use rustler::{Atom, Binary, ListIterator, NifResult, Term, OwnedBinary, Env};
use google_api_proto::google::bigtable::v2::mutate_rows_request::Entry;
use google_api_proto::google::bigtable::v2::Mutation;
use google_api_proto::google::bigtable::v2;
use bytes::Bytes;
use prost::Message;

mod atoms;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif(schedule = "DirtyCpu")]
fn mutate_rows_request<'a>(env: Env<'a>, term: Term) -> NifResult<Binary<'a>> {
    let table_name: String = term.map_get(atoms::table_name())?.decode()?;
    let entries_term: ListIterator = term.map_get(atoms::entries())?.decode()?;
    let entries: Vec<Entry> =
        entries_term
        .map(to_entry)
        .collect::<NifResult<Vec<Entry>>>()?;
    let req = v2::MutateRowsRequest {
        table_name,
        authorized_view_name: String::new(),
        app_profile_id: String::new(),
        entries,
    };
    let mut buf = vec![];
    req.encode(&mut buf).or_else(|_| Err(rustler::error::Error::Atom("encode_failed")))?;
    let mut out = OwnedBinary::new(buf.len()).expect("allocation failed");
    out.as_mut_slice().copy_from_slice(&buf);
    Ok(Binary::from_owned(out, env))
}

fn to_entry(term: Term) -> NifResult<Entry> {
    let row_key = term.map_get(atoms::row_key())?;
    let mutations: ListIterator = term.map_get(atoms::mutations())?.decode()?;

    let row_key: Binary = row_key.decode()?;
    let row_key = Bytes::copy_from_slice(row_key.as_slice());

    let mutations: Vec<Mutation> =
        mutations
        .map(to_mutation)
        .collect::<NifResult<Vec<Mutation>>>()?;

    Ok(Entry {
        row_key,
        mutations,
    })
}

fn to_mutation(term: Term) -> NifResult<Mutation> {
    let mutation = term.map_get(atoms::mutation())?;
    let (mut_type, mutation): (Atom, Term) = mutation.decode()?;
    if mut_type != atoms::set_cell() {
        return Err(rustler::error::Error::BadArg)
    }
    let column_qualifier: Binary = mutation.map_get(atoms::column_qualifier())?.decode()?;
    let column_qualifier = Bytes::copy_from_slice(column_qualifier.as_slice());
    let timestamp_micros: i64 = mutation.map_get(atoms::timestamp_micros())?.decode()?;
    let family_name: String = mutation.map_get(atoms::family_name())?.decode()?;
    let value: Binary = mutation.map_get(atoms::value())?.decode()?;
    let value = Bytes::copy_from_slice(value.as_slice());
    Ok(Mutation {
        mutation: Some(v2::mutation::Mutation::SetCell(
            v2::mutation::SetCell {
                family_name,
                column_qualifier,
                timestamp_micros,
                value,
            }
        )),
    })
}

rustler::init!("bigtabler_nif");
