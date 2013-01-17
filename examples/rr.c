#include "../src/hammer.h"
#include "dns_common.h"
#include "dns.h"
#include "rr.h"

#define false 0
#define true 1

bool validate_null(HParseResult *p) {
  if (TT_SEQUENCE != p->ast->token_type)
    return false;
  return (65536 > p->ast->seq->used);
}

const HParsedToken *act_txt(const HParseResult *p) {
  dns_rr_txt_t *txt = H_MAKE(dns_rr_txt_t);

  const HCountedArray *arr = p->ast->seq->elements[0]->seq;
  uint8_t **ret = h_arena_malloc(arr->arena, sizeof(uint8_t*)*arr->used);
  for (size_t i=0; i<arr->used; ++i) {
    uint8_t *tmp = h_arena_malloc(arr->arena, sizeof(uint8_t)*arr->elements[i]->seq->used);
    for (size_t j=0; j<arr->elements[i]->seq->used; ++j)
      tmp[j] = arr->elements[i]->seq->elements[j]->uint;
    ret[i] = tmp;
  }

  txt->count = p->ast->seq->elements[0]->seq->used;
  txt->txt_data = ret;

  return H_MAKE_TOKEN(dns_rr_txt_t, txt);
}

#define RDATA_TYPE_MAX 16
const HParser* init_rdata(uint16_t type) {
  static const HParser *parsers[RDATA_TYPE_MAX+1];
  static int inited = 0;

  if (type >= sizeof(parsers))
    return NULL;
  
  if (inited)
    return parsers[type];


  H_RULE (domain, init_domain());
  H_RULE (cstr,   init_character_string());

  H_RULE (a,      h_uint32());
  H_RULE (ns,     domain);
  H_RULE (md,     domain);
  H_RULE (mf,     domain);
  H_RULE (cname,  domain);
  H_RULE (soa,    h_sequence(domain,     // MNAME
			     domain,     // RNAME
			     h_uint32(), // SERIAL
			     h_uint32(), // REFRESH
			     h_uint32(), // RETRY
			     h_uint32(), // EXPIRE
			     h_uint32(), // MINIMUM
			     NULL));
  H_RULE (mb,     domain);
  H_RULE (mg,     domain);
  H_RULE (mr,     domain);
  H_VRULE(null,   h_many(h_uint8()));
  H_RULE (wks,    h_sequence(h_uint32(),
			     h_uint8(),
			     h_many(h_uint8()),
			     NULL));
  H_RULE (ptr,    domain);
  H_RULE (hinfo,  h_sequence(cstr, cstr, NULL));
  H_RULE (minfo,  h_sequence(domain, domain, NULL));
  H_RULE (mx,     h_sequence(h_uint16(), domain, NULL));
  H_ARULE(txt,    h_many1(cstr));


  parsers[ 0] = NULL;            // there is no type 0
  parsers[ 1] = a;
  parsers[ 2] = ns;
  parsers[ 3] = md;
  parsers[ 4] = mf;
  parsers[ 5] = cname;
  parsers[ 6] = soa;
  parsers[ 7] = mb;
  parsers[ 8] = mg;
  parsers[ 9] = mr;
  parsers[10] = null;
  parsers[11] = wks;
  parsers[12] = ptr;
  parsers[13] = hinfo;
  parsers[14] = minfo;
  parsers[15] = mx;
  parsers[16] = txt;

  // All parsers must consume their input exactly.
  for(uint16_t i; i<sizeof(parsers); i++) {
    parsers[i] = h_action(h_sequence(parsers[i], h_end_p(), NULL),
			  act_index0);
  }

  inited = 1;
  return parsers[type];
}
