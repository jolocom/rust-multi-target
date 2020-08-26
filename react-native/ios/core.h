// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#pragma once

#include <stdint.h>

// rust ffi

struct ExternError {
  int32_t code;
  char *message; // note: nullable
};

void signer_destroy_string(const char *cstring);

// ethkey ffi

// validates a key event log,
// see /react-native/rust/keriox-wrapper/src/lib.rs:7
const char *validate_events(struct ExternError *, const char *kel_str);

const char *get_id_from_event(struct ExternError *, const char *event);

// returns a new encrypted walet
const char *new_wallet(struct ExternError *, const char *id, const char *pass);

const char *keri_incept_wallet(struct ExternError *, const char *ew,
                               const char *id, const char *pass);

const char *change_pass(struct ExternError *, const char *ew, const char *id,
                        const char *old_pass, const char *new_pass);

const char *change_id(struct ExternError *, const char *ew, const char *id,
                      const char *new_id, const char *pass);

const char *new_key(struct ExternError *, const char *ew, const char *id,
                    const char *pass, const char *type, const char *controller);

const char *add_content(struct ExternError *, const char *ew, const char *id,
                        const char *pass, const char *content);

const char *set_key_controller(struct ExternError *, const char *ew,
                               const char *id, const char *pass,
                               const char *key_ref, const char *controller);

const char *get_key(struct ExternError *, const char *ew, const char *id,
                    const char *pass, const char *key_ref);

const char *get_key_by_controller(struct ExternError *, const char *ew,
                                  const char *id, const char *pass,
                                  const char *controller);

const char *get_keys(struct ExternError *, const char *ew, const char *id,
                     const char *pass);

const char *sign(struct ExternError *, const char *ew, const char *id,
                 const char *pass, const char *controller, const char *data);

const char *verify(struct ExternError *, const char *key, const char *type,
                   const char *data, const char *signature);

const char *encrypt(struct ExternError *, const char *key, const char *type,
                    const char *data, const char *aad);

const char *decrypt(struct ExternError *, const char *ew, const char *id,
                    const char *pass, const char *key_ref, const char *data,
                    const char *aad);

const char *get_random(struct ExternError *, const unsigned len);
