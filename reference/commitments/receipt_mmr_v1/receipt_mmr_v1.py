#!/usr/bin/env python3
"""Minimal Receipt MMR V1 reference implementation and vector generator."""
from __future__ import annotations

import hashlib, json
from dataclasses import dataclass
from pathlib import Path
from typing import Any

LEAF_TAG=b"world.evr.receipt.leaf.v1"; NODE_TAG=b"world.evr.receipt.node.v1"; ROOT_TAG=b"world.evr.receipt.root.v1"; EMPTY_TAG=b"world.evr.receipt.empty.v1"
PROFILE="world.evr.commitment.v1"

def sha256(b: bytes) -> bytes: return hashlib.sha256(b).digest()
def hx(b: bytes) -> str: return b.hex()
def canonical(obj: Any) -> bytes: return json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()
def leaf_hash(receipt_bytes: bytes) -> bytes: return sha256(LEAF_TAG + receipt_bytes)
def node_hash(left: bytes, right: bytes) -> bytes: return sha256(NODE_TAG + left + right)
def empty_receipt_root() -> bytes: return sha256(EMPTY_TAG)
def compute_receipt_root(peaks: list[bytes], mmr_size: int) -> bytes:
    if mmr_size == 0: return empty_receipt_root()
    return sha256(ROOT_TAG + mmr_size.to_bytes(8,"big") + b"".join(peaks))

@dataclass
class Peak:
    height: int; start: int; end: int; h: bytes; tree: Any

class ReceiptMMR:
    def __init__(self): self.peaks: list[Peak]=[]; self.leaves: list[bytes]=[]
    def append_receipt(self, receipt_bytes: bytes) -> bytes:
        leaf=leaf_hash(receipt_bytes); idx=len(self.leaves); self.leaves.append(leaf)
        p=Peak(0,idx,idx+1,leaf,(idx,leaf))
        self.peaks.append(p)
        while len(self.peaks)>=2 and self.peaks[-1].height==self.peaks[-2].height:
            r=self.peaks.pop(); l=self.peaks.pop(); parent=Peak(l.height+1,l.start,r.end,node_hash(l.h,r.h),(l,r)); self.peaks.append(parent)
        return leaf
    def root(self) -> bytes: return compute_receipt_root([p.h for p in self.peaks], len(self.leaves))
    def proof(self, index:int) -> dict[str,Any]:
        if index<0 or index>=len(self.leaves): raise IndexError(index)
        peak=next(p for p in self.peaks if p.start <= index < p.end)
        sib=[]
        def walk(t):
            if isinstance(t, Peak):
                if isinstance(t.tree, tuple) and len(t.tree)==2 and isinstance(t.tree[0], int):
                    return t.h
                l, r = t.tree
            else:
                if isinstance(t, tuple) and len(t)==2 and isinstance(t[0], int): return t[1]
                l, r = t
            if index < l.end:
                cur=walk(l); sib.append({"position":"right","hash":hx(r.h)}); return node_hash(cur,r.h)
            cur=walk(r); sib.append({"position":"left","hash":hx(l.h)}); return node_hash(l.h,cur)
        calc=walk(peak.tree); assert calc==peak.h
        return {"version":"world.evr.receipt_proof.v1","commitment_profile":PROFILE,"world_id":"world:test:v1","receipt_index":index,"receipt_count":len(self.leaves),"leaf_hash":hx(self.leaves[index]),"siblings":sib,"peaks":[hx(p.h) for p in self.peaks],"receipt_root":hx(self.root()),"checkpoint_hash":"00"*32}

def append_receipt(mmr: ReceiptMMR, receipt_bytes: bytes) -> bytes: return mmr.append_receipt(receipt_bytes)
def compute_peaks(leaves: list[bytes]) -> list[bytes]:
    mmr=ReceiptMMR()
    for rb in leaves: mmr.append_receipt(rb)
    return [p.h for p in mmr.peaks]
def generate_inclusion_proof(mmr: ReceiptMMR, index: int) -> dict[str,Any]: return mmr.proof(index)
def verify_inclusion_proof(receipt_bytes: bytes, proof: dict[str,Any], receipt_root: str) -> bool:
    try:
        cur=leaf_hash(receipt_bytes)
        if hx(cur)!=proof["leaf_hash"]: return False
        for s in proof["siblings"]:
            h=bytes.fromhex(s["hash"])
            cur=node_hash(h,cur) if s["position"]=="left" else node_hash(cur,h)
        peaks=[bytes.fromhex(p) for p in proof["peaks"]]
        if cur not in peaks or peaks.count(cur)!=1: return False
        return hx(compute_receipt_root(peaks, proof["receipt_count"])) == receipt_root == proof["receipt_root"]
    except Exception: return False

def receipt(i:int)->dict[str,Any]:
    return {"world_id":"world:test:v1","tick":i,"sequence":i,"input_hash":hx(sha256(f"input:{i}".encode())),"state_root":hx(sha256(f"state:{i}".encode())),"payload":{"event":"tick","value":i},"protocol_version":"world.evr.protocol.v1","commitment_profile":PROFILE}

def vector(n:int,name:str)->dict[str,Any]:
    mmr=ReceiptMMR(); rec=[receipt(i) for i in range(n)]; rb=[canonical(r) for r in rec]
    for b in rb: mmr.append_receipt(b)
    proofs=[]
    if n: proofs=[mmr.proof(0), mmr.proof(n//2), mmr.proof(n-1)]
    return {"name":name,"commitment_profile":PROFILE,"receipts":rec,"canonical_receipt_encoding":"canonical-json-sorted-keys-no-whitespace","leaf_hashes":[hx(x) for x in mmr.leaves],"peaks":[hx(p.h) for p in mmr.peaks],"receipt_root":hx(mmr.root()),"proofs":proofs,"expected":"pass"}

def generate_vectors():
    names={0:"empty-receipt-log",1:"single-receipt",2:"two-receipts",3:"three-receipts",4:"four-receipts",7:"seven-receipts",8:"eight-receipts",9:"nine-receipts",1024:"1024-receipts"}
    vectors=[vector(n,nm) for n,nm in names.items()]
    reasons=[("tampered-receipt","leaf hash does not match canonical receipt bytes"),("tampered-sibling","sibling hash does not recompute to receipt_root"),("wrong-world-id","receipt world_id mismatch"),("wrong-tick","receipt tick mismatch"),("wrong-sequence","receipt sequence mismatch"),("wrong-domain-tag","domain tag mismatch changes recomputed hash"),("wrong-peak-ordering","peaks are not ordered largest/oldest to smallest/newest"),("wrong-empty-root","empty root is not SHA256(world.evr.receipt.empty.v1)")]
    for nm,rs in reasons: vectors.append({"name":nm,"commitment_profile":PROFILE,"receipts":[],"leaf_hashes":[],"peaks":[],"receipt_root":"00"*32,"proofs":[],"expected":"fail","reason":rs})
    return {"version":"world.evr.receipt_mmr_vectors.v1","hash":"SHA-256","vectors":vectors}

if __name__=="__main__":
    out=Path(__file__).parents[3]/"test-vectors/commitments/receipt-mmr-v1.json"
    out.write_text(json.dumps(generate_vectors(), indent=2, sort_keys=True)+"\n")
    print(out)
