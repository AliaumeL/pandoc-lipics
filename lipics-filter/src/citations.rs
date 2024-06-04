/// Handling citations.
/// Better Citations (natbib style)
///
/// - parse [@cite]{.authors} as Author names \cite{cite}
/// - parse [@cite]{.year}    as \citeyear{cite}
/// - parse [@cite]{.p}       as \cite{cite}
use pandoc_ast::{Block, Inline};
