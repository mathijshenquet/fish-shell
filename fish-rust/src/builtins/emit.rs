use super::prelude::*;
use crate::event;

#[widestrs]
pub fn emit(parser: &Parser, streams: &mut IoStreams<'_>, argv: &mut [WString]) -> Option<c_int> {
    let opts = match HelpOnlyCmdOpts::parse(argv, parser, streams) {
        Ok(opts) => opts,
        Err(err @ Some(_)) if err != STATUS_CMD_OK => return err,
        Err(err) => panic!("Illogical exit code from parse_options(): {err:?}"),
    };

    let cmd = &argv[0];

    if opts.print_help {
        builtin_print_help(parser, streams, cmd);
        return STATUS_CMD_OK;
    }

    let Some(event_name) = argv.get(opts.optind) else {
        streams
            .err
            .append(&sprintf!("%ls: expected event name\n"L, cmd));
        return STATUS_INVALID_ARGS;
    };

    event::fire_generic(
        parser,
        event_name.clone(),
        argv[opts.optind + 1..].iter().cloned().collect(),
    );

    STATUS_CMD_OK
}
