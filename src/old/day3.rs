use std::collections::HashSet;

pub fn solve() {
    let mut priorities = Vec::new();

    let mut first_elf = "";
    let mut second_elf = "";
    let mut third_elf = "";

    for line in INPUT.split('\n') {
        if first_elf.is_empty() {
            first_elf = line.clone();
            continue;
        }

        if second_elf.is_empty() {
            second_elf = line.clone();
            continue;
        }

        third_elf = line;

        let first_contents: HashSet<char> = first_elf.chars().into_iter().collect();
        let second_contents: HashSet<char> = second_elf.chars().into_iter().collect();
        let third_contents: HashSet<char> = third_elf.chars().into_iter().collect();

        for item in &first_contents {
            if second_contents.contains(&item) && third_contents.contains(&item) {
                let priority = get_priority(*item);
                priorities.push(priority);
            }
        }

        first_elf = "";
        second_elf = "";
        third_elf = "";
    }

    let total: usize = priorities.into_iter().sum();

    println!("{}", total);
}

fn get_priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        let result = c as usize - 96;
        return result;
    }

    if c.is_ascii_uppercase() {
        let result = c as usize - 38;
        return result;
    }

    panic!("alalalal")
}

const TEST_INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

const INPUT: &str = r"LLBPGtltrGPBMMsLcLMMVMpVRhhfCDTwRwRdTfwDllRRRDhC
gNFJHJFgtZFJjZJHNNFWZWZwwDjCwSDhfCDbdwjfwDTTDT
gmQNZnZNHWnqmQpLtVLMBsPpBqrL
HlHldQtHlctzppdQtjdczHhJRnnhGNVmVRJmVjCVFCNh
LgWNgggZJZGFhCZr
DbqPswwMvDPqzlBNHtzfHdwd
tJgtJwwCtNvPHHPtHzDsdRTsBRDDWgWTgT
QhLQjLGjZQFlFZmnmGLDrzWfRldrTrzTBRWTzs
bFFmFZjhSFHvBCvCvJpb
MSGcvnvMGMJgWJDpdndZwBnppfCp
VPVfQQVbshZNZwdNDwNs
LtLbjmQRLmVhQtTbfgWjJgFFcrqqrGSqWg
fHfCNCwwHfGhcntntrrgHrQnrn
FVqpSpbPpjSVMjqvVmVvMzlzwJnbtnnlzQQlrWzJgt
PTqqRRPSRSmqSpPpSpRZwGCLGscCNLZZZTNdNZ
pQQQslVSVzzCQnZSlplzbLcHZHcrrrbZqFbZjbFm
gWtvPgdMDDtFDHHjJJbbccbrLW
MhNvwwDfDfdtvRQnpFNNTlSRSn
ZTnSnTTzqvFmVzvWWm
ClpCgltHNrtgsHdpLCHtDCNLVvQvVwVmwcsWQGMMQvcGcFcv
JmrgCHCNJtlmHmNhnJjnnnjJhPfhSJ
BgRRZTgHHvRTRmRNLNNhQWlmGFfJlWlhsQshpF
qPqSSttwnnzqqqwtVrPwMthFsJllJJlGhpJhWJQlhVQd
MjMwScnDPzcwjtqDtztnctrvgNZTTvCvLgvQbLbvjTBvBg
SWQSbbqTTbPcfMZSwZZwwn
dghjghmNDmGsGgdnfmtMRCLCCRncfc
pJDJNdsNMMhpssgdprBTBzWlpBWlllWb
TwNLNZTwWCWLwWCSTZSLzWHGrDHHPmGdDHvndGdNfvMm
BgpjtpgjBjVbRjQRhVsDnvgGgPnGdrmvnMDfrf
rhRjRssQJplRtVbpthblbbLSLzFCJZFqLLFWCzqcqzLL
PBrdPMtBPvCQBVBjCfWPqSHbszhGGnsfSG
JpmDwJgWJgNzmShhmfSGzh
pRwNcNpFZNZRWgcNplpjVCMVjdvdMQtCMLZjMZ
lDrcnnlLqLRcDDZRLjFVTHzGCLGVPzGPVWGB
pNwHpdmsNJsbpwsbzJTCPWTVFzzQTWCQ
vbhswdtdwfdsmtNSssHwvllvMcZjnjcnZqlgMDZglM
GVVtJGtzVFsVsDTH
mQRgcBRmRLnBjrtFjCCrHmFF
gqpBnlRpgZcvdSdlMdSvMt
tMSCNGSflffNhnnGqlPPsrzWPrTrVpWr
bZHbmDBQmbDZQdbDcRFZZBTTWWWwqVzszWjrFPVwrzqq
HQBLHmQVQLDdCggMfgMNLvNG
HHNDzNJPJPmdPcNGGGhnhwnVhCQBwBjQ
bsSbLfrLtRSLRSRRRsBwhCpfpCzlwCBVjlCV
zvvsvqLtZqLtzRsqTrggRMHJNWJgHHHNJcgWNPdHcH
qgbNvqbgmmZgZLvZqgnZzlpzpzHtVPzttGPrrnnl
jwswGjQDMsQMjdBwdcjCHVtcPVpCVCrPlVSrpc
GsFWBfhGBfDFDFWqNbLNqbgvqbbvfN
HgwWqtcqHNWgnHcNNCfvJCCJJfJGvnPfrR
sbDhZSmdBbsSdmSDdrjjffRvdjPrprCd
vvZbSFFlFHtqFqqWNc
ZRjnbRsHlncZGjTRTfFVSQBQppQvvFBHpF
zrLwMdhDhqJJttDQSldQVPQSlSfPlV
hCWWCzqWnmcZlRRW
HfgfQflHjWgRQRdRBWVsnbvvscbbbwvmbHncSc
tJGLtPPGZPwVvSSPhw
CLGTLZqJtMGqLDFFDZZJFZJpWjRpVNRWpllDpjlBfgVjlp
rhhGZZhLNhPmfJqvfLlq
dHRTHRHQQWcTCRTHmmjJgfqqlGmgWgql
CCwRzTRRdCCRSQwzRcppprZtrMhGBMZMnDSt
WfffvnSnfSBshwsjhlvGlh
ZHpFNTmppVmNzVVmmFMZzbwwjHGrGlPhCGrljbgHsg
pLZMmqVsZVMMVVscDfdtSSStqcRRdn
RhRbLzRLHLCPmzznHLbzCRTJhdTVSJJVSjdFFNFFNTJv
MGgMfpMsBgpnMtGfnfwBtDBjFVdNSSSFdvJSQSpTJdJjNv
lMsBgDMsblmRblnz
ClNcJZttLfLvvRQzQWwRQN
hrpMdqMspsrGDdMphhdMMMMHBmRWmSVrRVzVTzQBQvSmzVWV
ppHDMGhMMDbGMdDMGbgFbgbMlJJnjjZtZfLPcfcngZfPPfCR
ZRslLRgCclZLZzQghQhfrbfGbJ
pVSHpBBBBDVDqDBldVzfrMzQbfSTSJrzzJrJ
DqqHnBDlpNDVVnpnjtDtNjCvFLcsFFPZRcPsNNmPcFcP
LmLWSmSRNdcpcRHFHrWzWHbMbwZlZlPSbTjlwPbTPJTf
DttBsvhnhqvGGBhGtBVNBVqJlPwslMMPJwTjZbbZPTfbPs
CDthQvVNVFCHHWCFdr
RRtCWSzQZdRMrtRWrSztMggcGDfQTcfFTGqTLgGDLc
bnVhnvPHhhdJJBTLDGcDTcBvvD
pmbnhmPPmHwdCjmdrRtCdj
lTPzwhzmHpTvrDCDHJnsNN
tdgtbMMBbWdFbtqJCnsrqnMMDsrq
FjWdtgLSWttWtLSWtDWBjGGmwGlzTRwPTQGhlQQm
wcbnTtTppNLrntznTBBccCGrVldRrZqdqRCZdFZCVZ
JfHDgjgPPfRRgRlLRddR
jhDhhLMfmJjMjDbNSTzbbbtmttmN
CfGlvzpvpTjzzCWjvDlfvbbJbCRSdSRhsSQCMhdbhR
wqrSmrLHHNcLqrrLBNsndssnnhPshnsQwbnJ
NtcmBLcNVDWzjSvWtv
vZPCSCvCJffvVvmCmPqCSlDSscczHDRcwcHzRlRHHs
LFGFNnGrdQttNMFpzpMRRDslsJwsJH
gjtLnFBJrLvhZvCbZhqB
DBcjVFjDhQMSJVZbHZbl
nfmsqppnLfTnfmMmzppwgllSrbSHHtllqbtSwZ
TRzTnfRWnfdzWssfnRfRpncQPBhdDjjDCPcMQcCBGPPj
NSjWCHjNHjpPWPpSFWdtqBMBBFVBvqvJGJwqBt
gQllgDrnhQQDGRshRsZfVtVMRqwMtccVJcBtvRqw
DQrzrDzhQgrsZLrZjWSSHNTWCjjNGTLH
CgdcCFcbTbBzPgmNRmpptP
rsZtsvVvHZZzPmqVNPzNmV
HZjrwrjnjtHSHwDGdFhCdhWWJnWchCFJ
RMTqQMRJqPtBtGBPtWjN
ssHfSfShCwwbhsbHhhsmSfhSGNpCpNCjBBBLptcGtpzBBBWW
HnwrSFwffHsFwrSSjfHglJJlTgZdFdgZRZTDDM
pDLDWlDSlJDmzSJnDScRPLGGvqFqLPccGLgv
CZHfwNMVNjsHNNqPgcbcBbRQGQ
dCffZCjVCdCHHTmnlSgTlTSrlStp
bFtlLCvLlVjpCGPJndrrMMCDDCnrMg
hRsTwcZcBjZRJrfMDnsHrJnH
mNZqcTSSBTScNzVQFtGtjpFtjmGG
bjHdLrHjRWpDCtLzhzps
lZcGfTvQcQfvlqqcNCcBvVwtGzmzthmwmpthMDmswgMt
NcqflNQTBTTvvQSvqSVvQJbHPHbHCRJdndJPSHjWHb
CVmRncrRVrhcmsBgfmtfdJsJmt
bZHvZZDJwpWtdZgtGNGd
vSbwHDMFMJqPQqQvvSPQqpSwjRcTVTLjLRhVCLFLjLFnFzCC
mtffsmBwfwBDBmmsLsHqtpftGrMVMPSMPsVvhNvFrGPMvjNV
TQTQCRWjJcdcQQSPrhhPSvVGPF
cTRJCnldWJZlTgbWgbdbpqfqmppjmtljpqzmjpLw
NNPmrmPWmrSSNNPmnglghmCvLCCflh
LFbsDQMQFtQFHbQHqhvnngCftpcllptJgJ
bDjsGqLLdRVjPZPP
tgrbBQlbtRblwtRGrbCNswDDCsvFszpssCss
SJVMhSZfHvpdhphN
SMLpWZSSZMjfgGBgRtbQgljQ
HsHHNDDHzHDDjsVBBZqtWBrSNcPwQvccvvdhPclSrQSc
fGCFCgpgTfnTmgTFLFgccclhwQhwrzSwSwrCrr
pmLJGfMRpFmfFMzmgGmRpgmVqWJDqZqqHtjBBVDBBqqssJ
mBTfcfCCmpBCCSzNQScQSTfddhdtwgttjghNwGtGdgwGtd
HvvqbvMLnFZVVPjJGRGzGRjZtwgw
VFHFbsFHHSmzQBmsmT
ZNmZCmNHHzzmPPzlbplvhbQh
GDSwldfdvggPfLvQ
ddqrtlnJDJlnjScRmMRCFHTHtFZF
FPvglHSPcpNcFNSHFHNvZjdmbwdbzZtzsHDzbsbj
MMnBLCCWBJCnrCVWCBstTZdZmdTtbDLswTtZ
BMDnRCrnGhPPSgcgpG
nsbgpbdrjMdGqnNRRWWRww
tZZhPzCJhsJBtJPllJBCtCvwwcwwWLvWvwWRThcGcqLq
mlBmZQPZmlppbgMmfssg
RFdZTHFCdvjhgGnFqj
zQLtNQpzNNtNpDtDPWLNMmGfBcjgjlgjhBnvcfnBvfjp
PtmLsPzQVWzWDswCSwHbRZsGZw
nPsfnPsFhTGjqGnmQppG
RZhBbNwbBRZHZSCCHQqSpCpqqm
VMbgNWRWMDfhtFJT
RWhRPDhBHZWgZghRZwZgGJPGdncFdLcdLCjscFcjCjNLLj
mQfSrlfTVqmSVTTTrprfFLqcdLHsLHFnvsFFqnNd
TtQmVHmMrbMWRggRPJZP
TTlCTVTdcpBlcchF
ZLhwSMZhqhtqwqLjFcBvFmvvssGBmmjj
LwSMRtqMHnqhhRZRRtJSVTgggVPdTdrVbQDJgTPW
CGFFWFFVgjfzgVfcJCcgTCcBBWqSqMMBMBShhwMLMwSSMq
fmQnflldltBZqlwqNZpB
dvtnvmtRtsPbzCfTHjHcPzGf
hzshzfshVhthgMmRsFRvFqmm
PDDcZWlWBbplvmRRGtlvqQ
ncjnDjbScnBWZjDVfwjfrrVtwLjzhr
QRWvffVVGfDhNNjzGZLLcGGZ
rgtpSSHpPrHSspvNLFlzTgNLlFglcc
SSpbMHpvmwMQhMBR
dHLtBqPCtPBHNsbRNdNNsZVN
nQwntMwJWhwWjvcjDMlntRsNpgSbNNpglFpVggbSVF
QDhJWwhzJtTqLzCmtT
PSLqTqrCrRvCSJWLdLwdVWdQWL
zNjHQnnHjHznnbDMnMMMdVZcpZZJpZWcdJFZ
BntfgNbzfBtHzgnbbbPPSstlQSSGGrlGsrTT
QpBNsBzztgqVtdmp
jvrhGljRhSTlGGvjwjSwGjRvHVdqLttrMgMbtMMMVmdqqHfV
ChTvTvljmCsQQQnNsQ
CQCNSQHHgCtNHCNHHNDJcBJwLPtJBGhMPPPJwM
zRTqmsdRRzrmdzVRpzPwcjdwwhLjMBMGBBLw
hprmzRmblTzTVTVrlbrmVHNWNnCZFWNNFZlnDFSWgQ
hGGqwwdwMqsRDGRBzlvDzB
LTNTfcCFFFCcNHFFBzRSZRBlzHPSZdvD
nLVTFNfVVLLWnwnwdrdbhnrhrr
hlTpcDTpHmHwDmMbbdMMMGTPdGPR
ZzFqNSQqHvBvzzqjFHtvSGRRMPQsJGJWRGWPMRdRsM
BZjLNqNqzVVHgLVgll
ZHHBzSZPVqghJgSnBhqJRQLRRMvQpwZvfNQRMMMp
ctFCDmdDWmDGNRFMpRlwwQPP
PrsmDmCGjtcmdjGtVqBSjJhnSbHnnghH
QmZHTjmmHRmmdPRvHdVlPdrNNLqWzffbRtqpzfWtWsWNNW
gwMcgnMGFGCjJLqfbtNtzzssCW
DwMFGBwcBFjhBBhcDSJQQVQTPldTvPlVVZQSdQ
NRTGfNffLghStLRR
QlnWsdJWmnbWnVqWbWqHPSpmjgCjtSwhPjgtptLS
JWchnllHqQJzGTZfTcFNDN
VtdtcTVVCRctVdJclCVtpphpPhNGDwNPmThwWmgG
ZjZMFnfBqqMjHZHMzBnzgPGwDmhmhDPfQNGPQGfD
BbgsnFgMgMlVdJtlcVSs
tlBMdBnClhLJnTbgph
PhDDczqDGPqsHGrRGPWHGPzcFJNLTTJZLNbNLfFZgTbffL
sHsmzzrGmPrRDRHqhHwmjBVtllwtdMdBSBtl
QscfZsGsVjVtqGmlzvRMvl
ThJNCHPTDDhHHJTJPHmlSMTtTTlBvlnMSzqn
HhCdrHrCcpmmdVmb
WPPBPvRWzvhWhWzGWtBqBSTLDZhgFSTCDgSgZZDCZs
flbJmMJnjdMqNdfZZrFZZNFZgrrsTZ
nQnqJlJdlQMMbVnVmdMplVnnBwcBPGttzQcvtHcWwWtHRHvB
LLsmpJTWCJmJppCmgHCCLjbFtRFghzjfjcjcZttbRg
SZlMPBdBtQfFSbSF
nPqldlDwlBVnvdLWJVsmVNZCCVmJ
HWvNVtHWJjHJsSgHsHzsDsmf
RwZGPFGMQgzpTGSD
PZMlwwqhFPPZqwFhPwnFbMjWJNNBtWNVJlCJJWJjWWzj
frBSzJDtztfNVGwRzVgGhqsV
MPMmjPWGMMmPCQCcbmRwVhTgVwTTqjvRTLww
cFpcMGFplDHfBHFS
gtjhjLffmgjgmbgVfbNdqFJMJMNbbwrwqq
sWHHPSJsHzTZzTGsCdrqCNNddGdGFGRC
ZpzHHTZWzsSSnBBPsTBnLVcpQfcJcQVQDQfcDfQt
qMPqChqjQPRCMqlBrmGmLbPSsTbSvz
nWNHZFVZZttWpfHsGSbBGTbWBSGmSm
nZfpVfdZdtFHnwVHZtNwZhCJRJhcCdDcQhCqDSSCQc
LlwSlZrftFSMpfLCdltTmmmSDmJqmssDVJBmJB
cRcGGhpvDTmTDgsG
nNPcjpWbNzjRRcWhbzWjvnLMddMLCwtdtMttddtrCdMz
NszSsDCMSDzdZpCMCSMpNszfTvJhlvmlmrTfrhlhHPrmhD
FRWBgRjWwqFWQFBBWjVncjRTvJfvvJvVrHhmVrHhmrdJTh
wnwnqwRGFqdbNNtCGpCp
zgsBvPVVDDrDtDgt
nTHldmJQNTTfflcJNrQlHWpmDDFDFhWpWCLtFbphCm
nTTNMlNfHQZTQPGSzVVZSVPSwr
bPLbtPpwsJhlpnhnnLNNZDWhRNzWQrWWffNr
SczqFdFHSTFjmMSMFVqFGCWWNRrWQQQRZCVWgQQgrZ
dFdzFGHvjmqGMFwwLLsPnvBspnsn
lwJwwmblVdvjbbbJvVnlmjGTTNTLqffpqDJffqGLqDLD
ZtWgPtRMtQRQnTGDQNTTqL
gCztMgWgchHhvwlllbnl
cCwSSCVbqwCCWSbZMmGdtBllWBfdlvdt
jzRsJjhPjnLthJNNpmpvmvvMfGvjQpGv
nPHPFgRHLtCHZrqTcq
dVJwCJGCVrQQGTNtLtGm
hWWgDHBzWWWpZlhWBssLDTDsQTLLtswswL
gPhBHpjwHcljpggwwWqvbFvdCVRqPPnnqVRb
zRRRRNqzpQZNNRRmRcZscQcCDmCTTTDGfTbfGhrTCTrbFF
HMvMtjgtLHVlLVfhCGfrfhJhhrvh
LBgStjnHBjLVgggBgHndnSNNQdNWcQQNGZccwsccdQpw
jLRqmZNGtZtvZvHzPfCvSSzhCP
QbwDVHFrVbDVrDFbzPwSThSfddhWPWzS
rpnFDccHFHtZNmMmRntj
RFVdzzlNtrwSTltb
hHGcqqBcGLQZffHhMwSswSWGrnnbMStC
cgqLBgQgpgbbPbPz
lfcgglhfTvmlBvclbgztnSRtSmttwRJwptWR
FMjDjsdNDjNMQLFFLCMQdtwGGzRwzpGwzdWzzJpGhn
ZQVNsVZMPsVhCQsFCFsHHlqlcBZrHHfBflbHBB
vGGQQdwNCTJfQJHJbM
FFqmzghlzhgqjlFqzZhmhPlRgBDLLRTTcHMbRcJHBLcgRH
qFrPjnhZmqnhZZjhhmpPzZmtvbpwtdvsSCCsGwdNwvwNCp
nrFdSHScdRwvdvRm
NNpPLJJbNbppCvmzbHTbmsTw
fWLHPlPtpMNBgGQgqggQSMGc
BcHtrBcnjflfHslsrnltbTgvMwpWnnWpwwwCwCCRRW
dzGhLSSGDdPNgLLdPWTqWWRMqwRWpvzMMv
VPZZNhhNSSDhLNSLdFZBVgBbjHcgsgfrbBJbfs
VMnWjjWTnNNCzzhblbbjlj
FmHwfFHqpDrJzPQLPLbCDs
GrdFfHqqSmmwHSqHfpdMNTtTtZCMMZtTRggGZR
QRlnlTphqNfqdjZNmd
rDtPmGctFrcgDjJcNjvNJNCcNw
bgGDtgDbBWBSBVlblmVmsRMmLM
CcQTQTrrmfQQhZZBpZpSSZ
JFqSvLlLbWggDvDDFHjsdnshBZpjHBBhBW
FgJqNvLRMlMMDDblrtfrTCStmCVtNttz
MRRbbddqtHbMZbqMHHTFTFgwZglWPfgsZWgW
LCcLjzCNGNcvpvLTFPmzlFsfTgFlgs
NhNGcrCGrsrvcDpvVcSbtHQJQbnQbSdHMtJV
bfMfBFcWFsWZHBWRPQpRqdwmMpmddm
rSShvvVTNVhvVCCvThDlSvCwpGCmRmGQmPwmpLRLRdpq
DhRzzVNVVgSzTFcgtnbHnHbfBB
HsTGHHvlvvGTGlHBvlbZstrVrwNjrjVStwVVZR
PPmgcFJPFcFWmWMgdNtVtQZtDVDVdZZjjR
LLqWnMnmNvlBLCTzCT
qTttLqLvGCQqCDlhml
FJjzrRBrpjRWrCwrBrrwpRbbDzgghSmmNhPQhgNshmDSzSNm
bJBrbFRjBVnWBrRBnHLfHGfdVtvHttcCdT
mTzjGPmPPmPNjNBTvlJRlNJzZqrzrSZZSpcZqpgcgcggFr
QWCwwMwWWhVZFbpQDSpSJS
stMMsWwMwVWtwJTNNPvvRmTsNPsl
gGFFNWMMNFTBlLpGpSll
qvccssdDwDbhMhzwHLppTSHLrdBpBVLV
PhJhzhMJzwDJwhZZtZQJCjgWtFjZ
pGqWfqqGcspGqWqppHprpTrzhCzttMBCtbtJmtJbSBvWBt
QDnVPgVPgDCJBMhmBJgv
NlZwFlnnPLLlFwDlDlnPPFFHTMTdMZjTTcjsqqcsdfGdcp
HLzZfHWWQwpgVHjVHr
JlMlMGGDMtJGdtJhqtlccDgVCSTFFSCSDTggpvFTjSgS
JcGRMlthtlVNMJRfzWsPnQsnnZNZns
zVfvMpsbtQmtBlFWBZ
lLSrlNTNRSFRFhhHRmPR
dnSJjjwJJGwwnzVlvpszvccM
SmlcCrpnrnznGzSBBSfzNbtsQsWZQcFbWctcbbZb
JHgwJPjvdghbbWdDZGNLZb
JjghvvhRwhwJVhHTzmfRfzGSMrzBfnGC
JbCmrbnzmntnVJjbCHJJFQFvqgJgQgqLDQ
NGhhhhPMGhWsSSchWlNsCLBBlLFQCgqvgCFFgQBg
PdcNWWcdGdPssPPNTSNNtzbTwjntzbbVwtZpCVnb
tGNgtsNQHsJmwwzddmQw
hMhhDBwMhDDfCRRBjFDDTTWjdWmrmdWqjlmmmjJz
RSpSSBhppDhRncRLswZLGvtGvNcNtL";