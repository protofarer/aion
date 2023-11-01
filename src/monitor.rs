// Resident Set Size: number of pages the process has in real memory.
//
// This is just the pages which count toward text,  data,  or stack space.
// This does not include pages which have not been demand-loaded in, or which are swapped out.
pub fn get_process_memory() -> Option<u64> {
    let me = procfs::process::Process::myself().ok()?;
    let stat = me.stat().ok()?;
    Some(stat.rss_bytes())
}