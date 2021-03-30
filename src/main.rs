// TODO: mutex -> arc -> ref

// Send trait, able of transfering ownership between threads (rc not send)
// Sync trait, safe to reference from multiple threads (rc, refcell, cell not)
// types that are made up of Send and Sync traits are automatically also Send and Syn