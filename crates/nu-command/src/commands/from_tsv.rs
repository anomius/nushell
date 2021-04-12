use crate::commands::from_delimited_data::from_delimited_data;
use crate::prelude::*;
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::Signature;

pub struct FromTsv;

#[derive(Deserialize)]
pub struct FromTsvArgs {
    noheaders: bool,
}

impl WholeStreamCommand for FromTsv {
    fn name(&self) -> &str {
        "from tsv"
    }

    fn signature(&self) -> Signature {
        Signature::build("from tsv").switch(
            "noheaders",
            "don't treat the first row as column names",
            Some('n'),
        )
    }

    fn usage(&self) -> &str {
        "Parse text as .tsv and create table."
    }

    fn run_with_actions(&self, args: CommandArgs) -> Result<ActionStream, ShellError> {
        from_tsv(args)
    }
}

fn from_tsv(args: CommandArgs) -> Result<ActionStream, ShellError> {
    let name = args.call_info.name_tag.clone();
    let (FromTsvArgs { noheaders }, input) = args.process()?;

    from_delimited_data(noheaders, '\t', "TSV", input, name)
}

#[cfg(test)]
mod tests {
    use super::FromTsv;
    use super::ShellError;

    #[test]
    fn examples_work_as_expected() -> Result<(), ShellError> {
        use crate::examples::test as test_examples;

        test_examples(FromTsv {})
    }
}
