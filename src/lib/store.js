import {writable, readable} from 'svelte/store'

export const APIserver = readable("http://127.0.0.1:8000/api")

const localSessionID = localStorage.getItem("session_id");
export const sessionID = writable(localSessionID || "_");
sessionID.subscribe((value) => localStorage.setItem("session_id",value));

const localUserID = localStorage.getItem("user_id");
export const user = writable(localUserID || "_");
user.subscribe((value) => localStorage.setItem("user_id",value));
