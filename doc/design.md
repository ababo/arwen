### Main OS design points

#### Not UNIX at all

I don't want to create just another UNIX clone. I want to try other ideas and to see where will they lead to. No POSIX compatibility is even planed.

#### Single address space

No hardware memory protection is required. I will use *Rust* language for system and user code due to its memory guarantees. User applications will be restricted from usage of the *unsafe* keyword.