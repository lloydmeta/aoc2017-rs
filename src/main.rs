extern crate aoc_2017;

use std::error::Error;
use std::process::exit;

use aoc_2017::day_1::*;
use aoc_2017::day_2::*;
use aoc_2017::day_3::*;
use aoc_2017::day_4::*;

fn main() {
    match main_result() {
        Ok(_) => exit(0),
        Err(e) => {
            println!("Something went horribly wrong: {}", e);
            exit(1)
        }
    }
}

fn main_result() -> Result<(), Box<Error>> {
    println!("*** Day 1: Inverse Captcha ***");
    println!("Input: {}", DAY_1_INPUT);
    println!("Solution: {}\n", sum_match_nexts(DAY_1_INPUT));

    println!("*** Day 2: Corruption Checksum ***");

    println!("Input: {}", DAY_2_INPUT);
    println!("Solution: {}\n", checksum(DAY_2_INPUT));

    println!("*** Day 3: Spiral Memory ***");
    println!("Input: {}", DAY_3_INPUT);
    println!("Solution: {}\n", steps_to_centre(DAY_3_INPUT)?);

    println!("*** Day 4: High-Entropy Passphrases ***");
    println!("Input: {}", DAY_4_INPUT);
    let passphrases = DAY_4_INPUT.trim().split("\n");
    let valid_passphrases = passphrases.filter(|s| is_valid_passphrase(s)).count();
    println!("Solution: {}\n", valid_passphrases);

    Ok(())
}

const DAY_1_INPUT: &str = "3893445835429722678558456317563893861752455542588369533636585887178232467588827193173595918648538852463974393264428538856739259399322741844613957229674619566966921656443476317729968764183945899765294481327998956154956571467872487576314549468261122281384513266834769436913544431258253346374641589492728885222652146158261225296144835682556133922436438188211288458692217737145834468534829945993366314375465767468939773939978272968388546791547526366348163672162245585168892858977723516752284597322176349412485116173844733679871253985762643852151748396593275274582481295864991886985988427966155944392352248314629138972358467959614279553511247863869663526823326467571462371663396188951696286916979923587358992127741723727623235238531991996999181976664226274715591531566495345212849683589582225465555847312199122268773923175183128124556249916458878785361322713513153175157855597289482439449732469754748544437553251412476225415932478849961897299721228198262823515159848941742786272262236888514421279147329383465929358896761449135917829473321834267122759371247338155787774952626616791265889922959653887288735233291968146648533754958199821789499914763279869931218136266492627818972334549751282191883558361871277375851259751294611921756927694394977764633932938573132221389861617195291742156362494769521829599476753198422283287735888197584327719697758442462886311961723849326959213928195182293316227334998926839139915138472514686689887874559367524254175582135318545912361877139367538434683933333264146289842238921989275112323681356256979576948644489986951538689949884787173194457523474156229389465725473817651516136514446513436419126533875125645855223921197481833434658264655912731133356464193251635637423222227273192628825165993827511625956856754776849919858414375874943572889154281862749595896438581889424559988914658387293414662361364793844213298677236787998677166743945812899526292132465751582925131262933636228593134861363493849168168765261647652342891576445292462341171477487223253795935253493869317616741963486473";

const DAY_2_INPUT: &str = r#"121	59	141	21	120	67	58	49	22	46	56	112	53	111	104	130
1926	1910	760	2055	28	2242	146	1485	163	976	1842	1982	137	1387	162	789
4088	258	2060	1014	4420	177	4159	194	2794	4673	4092	681	174	2924	170	3548
191	407	253	192	207	425	580	231	197	382	404	472	164	571	500	216
4700	1161	168	5398	5227	5119	252	2552	4887	5060	1152	3297	847	4525	220	262
2417	992	1445	184	554	2940	209	2574	2262	1911	2923	204	2273	2760	506	157
644	155	638	78	385	408	152	360	588	618	313	126	172	220	217	161
227	1047	117	500	1445	222	29	913	190	791	230	1281	1385	226	856	1380
436	46	141	545	122	86	283	124	249	511	347	502	168	468	117	94
2949	3286	2492	2145	1615	159	663	1158	154	939	166	2867	141	324	2862	641
1394	151	90	548	767	1572	150	913	141	1646	154	1351	1506	1510	707	400
646	178	1228	1229	270	167	161	1134	193	1312	1428	131	1457	719	1288	989
1108	1042	93	140	822	124	1037	1075	125	941	1125	298	136	94	135	711
112	2429	1987	2129	2557	1827	477	100	78	634	352	1637	588	77	1624	2500
514	218	209	185	197	137	393	555	588	569	710	537	48	309	519	138
1567	3246	4194	151	3112	903	1575	134	150	4184	3718	4077	180	4307	4097	1705"#;

const DAY_3_INPUT: u64 = 368078;

const DAY_4_INPUT: &str = r#"
nyot babgr babgr kqtu kqtu kzshonp ylyk psqk
iix ewj rojvbkk phrij iix zuajnk tadv givslju ewj bda
isjur jppvano vctnpjp ngwzdq pxqfrk mnxxes zqwgnd giqh
ojufqke gpd olzirc jfao cjfh rcivvw pqqpudp
ilgomox extiffg ylbd nqxhk lsi isl nrho yom
feauv scstmie qgbod enpltx jrhlxet qps lejrtxh
wlrxtdo tlwdxor ezg ztp uze xtmw neuga aojrixu zpt
wchrl pzibt nvcae wceb
rdwytj kxuyet bqnzlv nyntjan dyrpsn zhi kbxlj ivo
dab mwiz bapjpz jbzppa
hbcudl tsfvtc zlqgpuk xoxbuh whmo atsxt pzkivuo wsa gjoevr hbcudl
gxhqamx dradmqo gxhqamx gxhqamx
yvwykx uhto ten wkvxyy wdbw
kzc ndzatgb rlxrk hfgorm qwgdky ndzatgb rhvyene qaa wxibe qwmku nmxkjqo
qwx ubca dxudny oxagv wqrv lhzsl qmsgv dxs awbquc akelgma
rrdlfpk ohoszz qiznasf awchv qnvse
ggsyj czcrdn oolj sibjzp ibzjps asp
vbcs ypgzae xcvpsr ptvb leoxqlq zmpt fhawu yfi tvbp
ejkr qlmag nsz jwpurli nhsml asksnug mes
kkgkjml kklmgjk kjgpx iquytbj eccceb mfv iuyqjbt ovv
uoklkco zzey sdfhiyv ytdeg
azr mjv raz arz rdfb
pir dafgsah dafgsah kndjbml estcz yjeoijp kkcws ebq puwno
iqymwc tac vlqc bmnkz xustm leqi
gwdjed cfha axz xjuq
abfjsg pahat qlj zan qsfn iozfys jnvu bis jakggq
afwuejn zrbu zurb hrn lwvjb jnwixla aufejnw
vkqn cuzf humhriz webnf uzfc zfuc
eznxd kgbfy jqyc net vzfege tprzyc
mqnapzn vrgw ilzp vgw
aie zkkih fhpwu bbn fhpwu wvxxgmd
ksoasrn yll mvdjxdo wydymx dmodvjx drnjlm tcjpjhj xzakb wrsbuwl vaygdwf rsasonk
qahbh tfhkl apdqqpm tfhkl nsox
xkelwve mvdmesj xrto tgku gkb bpe
nni nyylpu cyusxe zydeyok yokzdye xiscesy
itwsfr eqwrx igqkvif whklwdb
lpa hwci suwqfln xis sfht lzek ajecd
svpf eulute eya gvmsd app claria tjtk zjt agdyemi bixewo
gmzglxi zlgouy bejg kte xlf giquj mjeq ivjkw ktbhaga hoffyrt
wwjy dtf ftd agei yde xhbfo fyridy
gexcy hetkz ufflrfi frifluf plb kqre etxo elg henqy fspm
khaemn buec ichau wxctsxg
cgmv ujyvcuu jta yux ccx skrafkn cmyc yidqhv ltb ycnajry zitq
ybsahqn pio veeze vdztjz iedou pio sue ijbz gvqncl vpa ijbz
hkfi xzrsyke hikf mxolx xlxmo ungfc tst xjzd
tpx ioprco qixlv ipocro
oahmwrv homvraw vws ntmbdvx
fxlg wnuz ogt bxgtul vmfh nwuz glfx tgxdq bxfv kajuh
vrhqn nrqvh tgogb vya ragbro ulrz uava kexoi yav vkfe
bxxy tyxgxd oabsud bauosd jlch bdmrqq wqhjwb ayblb hclj
sfzsgsc sfzsgsc jbrvh sfzsgsc bdhy
wixleal vhnqbfw qwfnhbv woco oowc
exkkwz wekxzk krxbua nshxqgh
gkn blxgui nkg gnk
otsa isqn otsa isqn
ude xedl ude xedl amkktp
teroe yuvbd inf mpytuvz xiq xqi ovqetn
zyq ybeifwx fvoqp vhoduy bcq wbxl
zymiid vafcqv vjbmekf lgxkma bjti qfavcv iqp fnbu lakmgx
rkaqvd vylkh jfdxh imxxg bbrt imxxg rkaqvd
yajg qnhhs bzmb eyk hijcg tkij iwr jvwp dipzd jvwp
btzhw zttheyo ravsbz bmbba majoe ykrs tbxqf tai cgsvpu srbavz
vsyczfs ymg vsyczfs wxlwaqb oouio owek wxlwaqb azvbqiq
ghrapd ghrapd wisq wisq
znmleu aztnkbs wxc gycxd vqenhh geqyo rpjg
kxbom gzz zzg zgz
dfsesc okwb dfsesc okwb
egpwqbe djlk xpkxa hoo eepbqwg
nxdfror yfhkhn zgea fkspva rjgg bnmq ddsf rjgg gkinm
vdrxfom wbdwu dhkt xtvzc zjobo aqvgrt
svddsgz mhcrbcp wmpd mhcrbcp klim ddlnxv wrdftrc ddow wrdftrc
obxr wscs new brxo wen epns cvjvxts ilnc
rwezl vmbut kgblt xfg vnhlebq nzqdzxm ynh wokrezy zub nzzqxdm
vephajp bzupele mltzglh sbgn vephajp lhve mltzglh
slajp kyaf vlnvy srfietn ayfk inaufex fanuexi
vazwg kjg qanzso ptuu vvlwq uupt kohhql jkg
xmmmpky rbqimi slvxsf tlcwm pbf pks iucx rbmiqi
irkup jvu tkeioz avdu suxamf
tmgih ldca jswka dblzzt rap rgqyy gyrqsk nnnn pok
pdbjhrl gsvxbqr nqfkhtc ngn okbgzd pdbjhrl oostjtm okbgzd
mzqfdat dujh aeplzqh acbguic vlzdt amyszu amyszu jqecky bhl hjqnimq xoy
dszafr bqampg epozj sfrlpe dszafr wfurku sotjpg wagtnxy
jbmo jbmo plbfkvw bkc jbmo
ehelldu vrid unnf vrid xqiu tbibjyi bmbpsmq tpqyefx xqiu
rpgm zzbj cjgfdyb bdjfgcy rzqecd miyvfbu aqlkagf hsuxwgl
gvywzp phvnd ypwypbm yfelxx egbr lcfyz hecdhkj xxfley
tsmebin tbsnmie mkijj ijjmk
cghxrqs vzxle wrfghv skighgt zviteab plcrgv
ezdirp rxkw horcek qcgny inx nikb tigzp
eidk sactjci sre vkapava owvf eyds eyds
vvjdm uye tjixj azklizl pnb
tcrimv xye twii xye twii tad
mtxcg lwjxdj zjudqu ekoujd ysf ajtfta dkj lwjxdj
aowhmvv kkic kjize fnohl ukx remfmii usbp
wkossu limxmhp xnoeocb wkossu lnrlqf kjozfg xeulstx sjncsw ekaimuv xnoeocb sxjegcg
lsfe zpewzlc yhjyiay lou ukhi lpwezzc slef zvtidgg kdeseq enka tfvgudr
ovfsa vuv tbtorv tbtorv gmxn opspw lli mfzvkv zlyhr oznalr
kugrpw sduq rdc ciaxwir ylnzwec kugrpw sduq
obevuau thu jpyfvof rpawwo obevuau gsvoutr quiaei
xpgua pbxa pxgau kdan
ohyzqk abxgg xozgai nib axozig bni fucgykm jpkswt
jrgu dmozts jrug ufpho
qojzue uzeojq txuhj eqjzou
wcvj qwlravl niyxf oiaptlk wlxnnzj jgdzap jgdzap lfgn bdt sfga adrypo
ylah eedu rvwdpmq eedu ylah
quages kmla yjqua dzxcfam srjag wujmcv qujya ssaol uzdwi
gdsppz yqxlish yfkjbbf ecnzu ejvtv cdjwre
slsls pcmrq zax btrc kliv ntho gymkk kkq pcrmq mvnw sjfegpx
ryz jfw eki wvibww qdzylg whbagp ffrfjg wdhnqpm hcrz
tcjqfh tmvzp mpztv vpmzt
xood xutgof teqov fqyyub oakm rzaheiq
axagoq jawbz sexucp sexucp atenr edekcwn edekcwn agl ecj gbje gipivfq
poqv qopv bos flhghs gshlfh
rxd dzphnb bwmna vxd rxd sbk kuor
kqeelq jqbyh xczqzqe jbkmx kelqeq xqcfqn
jdfy qzjyz xvqyo jdfy xvqyo
vyoqyd pwayqag eygmdt smakwc veftikz fzeikvt
aozgkne mpd mktgoew eepp zlwycr eepp hswbxcx nmi ddnfr eepp
dgpfp cfhhqdx vjrb uyimbm byx hfdhxqc
fxq jcouwfy uhuao zsab xjao
noudveu egxyuqw hmnnv vovt wmqkx syatiac whkd
gxyzk opgb kjxp delavq hsnvk kfn irkcfq lvc aadcwy opgb
exuiupk ddiiyvm nsggpj ddiiyvm nsggpj
hhjby rfejzp akxzs nconlt rynivtq voloj qwhhll ubvx yxuacz miuwxh ppe
uspqvx supvxq cekv niddfuw
optzcag sra ajs ozacptg yxkludq jjpvldz mxo mox
dko qyec iuxbrbj dlz jxribub
ywlyz vipfh amsfr fwiozi tahjov tap rsea zwlyy oqdyfbo
xeqml jwy eguc bvzvh
crp mxsihvo wwtg gsypx mxsihvo qpfw looca gewvy zjqki tdkuxo crp
mqlnzm yihsvrl hhtwcv kigymqu yswki hksk vbiujq xeqz irzcq cpnz
zxhfsw uuyhwid nzabem mmfk wszfhx shxzwf hqnrvsq
hfjajdl qwmk hjdalfj mwkq gqbku dsszk
fbiy pujq htgaqqq yro ztpe yiufb fnpi ggchdgz
sixq jsboan eoie msdrwzw sixq njsrc sixq yimqoi
pbxgv kqmi hjuk bbtrlta bqwm bgehofj ainqhm qoypsil manhiq ogebhfj lvmuo
wnax aen fthpcke tcz yadjmva mydavaj rcfkc krfcc
lkatiw zxliii usdj oopxl yylv bkjfy gtlyjv usdj muqazdb
yqonaxv wqnvoo hfpll oyxnlfs fgajc khhtzr hfpll gsvvipz wbjxsnp dcdikt hqw
vvuv kspmnz zvmryqd egvuz eazkhz kspmnz
xgq dziwiym gsl nbzmsta ccbzn yalof dbbugq aar ywmbvk yofla dcwb
qrtyhhw xeyo vlym ulzzbl hrxyb qeyu jqdkewk oxye evaxz kybc bssyt
eqrf cfyy kwhohw ozg jsc egz jsc
vct cet ixxvmz ibhvndq eks dpi jzfwdqv saeh jqzdfwv vwfdqjz
vus vus kitvvgq wpi alfncf gzj oxcy fith oxcy ecbsr
uacculk guwhwdp cankcv yswy bmby sve dvonm nran
ydftm wszgaex rgbw otd dbet lhsxndd jqfyx
vhawg hwagv uagy fveik nrsew zujw hawvg dzfmt agzgw
uqdj talb uqdj aizyuqm
pbbejee szdtohv tycfow xwne qzlqy dxcwejz pqdqrc wfyotc gdqt uxaeug wtldm
hmzmd oyp pyo opy
qwdh kwpll kwpll zsbez uxg klr uxg
myqr zza kqpcos adsql eumunrv qlaeumx
acyye xvdewe nwkhuz bzcmx asw ostiwk mfzu nwkhuz
memq uqadd kfj dses lrxb hxygp dsse bxbr hgpxy uavrar
mjmk lsdttuz qjkg yfthmkn pram
pctfq aly usim shihap uims xkfgp ksrbn ifvsyl
cdma nnnu hdm dhm
kpt upgsm ohvrrqf qwps wjcbve ohvrrqf
wowphgb nteme otizypb eumgvb puoctli opicult wbohwpg
fppz ftkql sbut lkqtf svif viqhlnn buts lljhbd
oqk uinby rqy vbjhf oul hzfo coca glpy brjy yglp qnvhvei
sbbwr dnyrux gpikv nsx aawyeq uhtucwq rhxzy jgx bdgdrl dnyrux lgfgi
agn mljz hgmglem popu jtapub agn
ehfpgr bnobvg bnobvg bnobvg
ozgzedn godezzn art atr
urz rzu xzyc rjhwi kgiodi doiigk layr dwbxu
rkcbav pnp bpsmm ifivfe csouqpw fyswzbd csouqpw bnjt rnnoxed
hpjgtcc ctcpgjh cchjtgp lxn
cinokbx uyaz uyaz uyaz
bphfwad bphfwad bphfwad yml izlhlb dvjvo jeropar
ocgftcl wshjk zbinw fcotlgc xdj nwibz
zbze hllno rmq invd gupoxr gwumc vnzj fcvvhjo dnn sfsxw
oqlhkz hgf yxiahks vhzvl ayshkxi irmwkmq
apeqic ahwu abxjrd tuwrd pynnil eohmlgo lafx ybpofe wbznxv swuafas
cpg jpsfo jposf rer ixeydpz
rhqrwvn wrhqnrv xptms jhc rnqvhwr
zfpl tukxzda lifkqtd ynfuno cttx ctxt tlqdkfi ueswv wbipfbe
eblw bwbjg fuu qqm qtv qtv isbl denfcb
ick yqwcffk pvcchd apkjyc ouu uyfe nplid ick caqscs sddkx
rtzh idn snnw xmlou idn kdhenl rtzh ujwttl pkynkhe
dnwha fpv dnwha iqi xggepo dnwha
yjvk saay enxqhw wigoah dzasyr nnt artl iqwia jpp xmfr hwigao
ryt heenuai ytr gqew hyb byh wdurx kmd adgjz
ypdqeji sfkkfhn stms cdmyh nqllx utiphia gxbx zflhtgo yurztx eni
pwlhlt lhlwpt rfkvlgr tucajej ckujc ntcyae xestygt eshmggk
gtfb codwc vjtli ffmjwx ruoekt cylrm ktroue dfaxzvs kkgejzi ewucgu jyatrum
ersbag cod xssha aqzbe kxu bzghhqk pbs bizvqk bhbbd bps
vhci ypxf bxzor unngl xilpzpk civh nykora vchi
cyb cceu negnld nbcfs pxsjgh xah nbcfs nbcfs jabpxg wtanv qhztvr
cljgzkn lrdeina hrjoz kdgpn vqkmpal nivk scvnu vzuausp
nif fin uxjbip xxztsn yyo opueh zxs qnso paedey hsd fttvqdn
gbnkmpr afo aof ryyudy gbmpnrk
uaa npb dkit npb buadan esv npb hwrj
hws dfgq fcyty qszhu chyxxl ytmpb azxl jrsn boqrx
hkzlnkd fkilvog xbubu fbgbp
fgi inmay uliytc vgkcw qsoe uliytc isjhix oyir ocaq
qrzkpm dpzetbr zommsxo cixg nwjyvp bet wyjpvn cgxi tsncd
uvlf lufv ulfv cigl
uwwf thr kdq fhjmty bvxue vcwwmk kdq nzajq bxkf
qcwduju idxaja qcwduju idxaja
fnarz pstzfne nco qzf kcevoo qwx csvsxga pstzfne
twug xrwy uoctfl bkh yxrw
unpdnbe apf cvm bpullu fyels tjpri jyw unpdnbe xfyekay vhk zpyb
rbv psirdv psirdv mnjrp qpwc vicismd qpwc
zjj zjj kesyhow eqcfqy vqy
zazpd gmea aobl dcs mage hqjdpwc bvxr srw
rhcdb nzsa jgcgc rhcdb wxs vsvvptn zvckqo wxs
unyet prchn fiwter wvgknes dvzbxfs ufet neuyt fczlrx bpocdci vdsfzbx
znfev fwrdarx knqkv ojiv ojiv fwrdarx
tbtlo hdashg kyspxm ypmkxs nmrk
fzr zqxaszt frz xzrre
shueb iraetk uhsv duvah uhsv zstysc nrfllbc emrknka
vzkrmp mgtkjnw njr bwjgp jdwyyhv yudha wbvmx ewu urhiioq
yjq xxr swvm aipdj apjid tfsq gfqg izrvhev
iljuqt fpo fxadit iljuqt iljuqt
zrj poewso vsje bsprbmc vsje yfwf ybu dmkqib ybu hlrpdi ymh
apxaeq bgdm mqty whyay mnuzfgk awm bgdm mwwi ekw bgdm
dpdbfkm crrg mkph kphm grcr ukbk
ilqm wroz mqil qlim
pnlx nwadw uabelu rueamxr wjer uwge jwer ywagrx
akuil nkh oitq werli werli
fkmhcr ieoj xfsa xfacoeb tcg poomcme vck zmpc djcqgkf kft
csyk qni hqfrye zyyxz ggynzad pjpokmu bigqa qie
lkpenw zyllii qtbvdq zqnu ichftg xazped agl irhlbiy snlwfe twliar
acsrba dzz ivylbl rfcdd rfcdd qcg
zbui fomvpx zjhmgl sivtffu xuhswzt fzeid tgj mzok mozk afbhuje tzswxuh
nupjiat fdxkbn tuatp jhdfnub yitdk yghqw nupjiat ibi edfv tuixw auwjm
focht mnprh tljj ayp
pjdnl uaoworh iqm gic dqlu spn heuymio
kvg ferrvhp unvzsd qdcpd rji zpch
nhvay chuzg pyhdd hnmrnq zeidhf pyhdd ohy hnmrnq
boa sau gxh grx
gwo utwpd zcsrx gow bnm
xoqniyd hmithl xoqniyd hmithl
yqqsbzo stca zcsjnqf skbueeu tlbkef mvqbg igzr wujuz yqqsbzo kkfe
wgzuepu wge fkrxuag csenx tkngoz wge azueyxs
get xiryxs xiryxs xiryxs
wammvx edy hahetl xmvawm dye
lscxxgi anmax quo cqprwn imocarq gnbfhe rcnqpw
znpmid yaluvzn ydm ckh uhso rrk wbby lwxsu
atppk byf dzz uift nqejgm njgeqm
dtqmy iog ahub habu
hkthdwt pfxlwsu hkthdwt hkthdwt
tsuiue tsuiue yais tsuiue
swooqmp rqrcs ngr vujrq inuu rqrcs
dhu zxdfiyv xuz xuz mgaty mgaty
kiiiz zco qdv vfgkj rders zco
trszp havbm redpeqk gktp ifvzvwl yfoxnm tzg avzd otiouso eks lqlutwb
cfiru lpdy kpeas mdc lxnjjqz nqyyb xkjsug rcifu dln
jga ijgkjo qhbnupb ofzqn iokjjg gaj lrh pkynrcr jgatk
bexwc tat tat otsngaa
feh mjxbs ehf cyfhlv vvdgdu hef
njlvq ojwaes awsejo ktyvxd qeyeze bpoaj ulgngn zyeqee kqc bsdzzvq
hbfp vnhs vnhs pko pxnxgm
bmy bzpn bzpn bcfep
cju nqjy yjqn bbrj esgzw swgl bjrb
cxvrshm rbglkyv kqwzcyd azqr ckwbbew fhgqv nfk lactzh ssqpwbr wbewbck
ptcb gqkb apcc okl jbbgk qni bqu slydyo qhh dqd osv
zbisefn bmxcljk bmxcljk arkamus vpq uxuwvb
ksik xbzk lahh ctfur sxh rduokr xqou zwbgqsp skik
hwhmfk hwhmfk bjpxzg qqftmu ijyv igvayf bjpxzg
askxqew tibx pqaczy fhzyec echzfy cezfhy
omzyy mbzfvsn kkoff qgqn crnnkn krx oqp jhn anb qte qxt
jypnwn vjbnbsl axf pldxbq pdoy rmxcvig cpad yhah rzqewkg nmzkkr erjo
visidzp bujlfn xuomjj mjnqn wgflg skb
oer oer lfi zyqnem lfi guljz
fannhwu wafma gcje cvcia qwyh ugtbpa geufqg
kwtjib pqwai tdmjj kuxr euzl rxuk
ovi splc hflutgw hflutgw
gvel gelv aeiygth elvg twwr kivxrrj jkmqa
bas ylxbdgn yliv pytkhq haujsyf fggrnbc wsgree rfnppcx key gvdzgfy evdtrrz
oblab wpgm bpyy xuroy qhb adqko
hneb law uzms fhhk yjymdx wjla ixfh yblh
qlvsd bxsq hjaq fuwspzu hyshq idbabc rqcih ilixp wft rglf lmqm
qdskj two ckd qdt hzjvd woo fmmuw
kumc zywzq srafcbb ihfu kfvav
qlkkrq qlkkrq qlkkrq qsc
hob bpecik zqtrfz iqizeu plrer epm zqtrfz xrekeql xrekeql
warszd sxyyorh sxyyorh eztjf warszd kszp
hjbrax liumjue liumjue liumjue
rfnqd folmiu dlicln pdyk uqd rfnqd
mjdu lytfvya xomdujn leaqiyc lgemz lihfnhv zgeml koukz luqda
yqsz zedjmwn aep qwbhd yqsz
etg rmovps abizj yqr kib
yznxec sfqkd ofkzep njr hmeym nsh xdq
ryoyq heoo zuo udvfev ehoo axcnbpu oeho mfenmd shrebzy
uaeh jwllsjp frkhqsy uaeh
giofw hwceb euikqp ldmb kqpkxwv namazcg hqyyzgs cglsqux
qledbd qledbd kbwo wgfmgp
olbsca muxw nxs locsba
gbxxgj xlzm gws pkpwy ofkxb sykhdo nbhrv
najr bfk tbqkm hxabe nvr mdi dmuujr bfil nyripr zcydzy
kiczhcn dfgylw yzkwk nytijj pceu yukj ekaol xpb uep
acyyxn rwczsud acyyxn payiek inusyb rwczsud
mzssokx bshs bshs ocrvlug nzsgvch riejkrd jkj mpmdgsp kvixdfq msmmx
uaxy wpvhf uaaq ranp vfhwp iik kii nvh
shecxef nqpx jly dzm qvmpu kxg hdg
xembm yzevult ljrllc yrlskyk zas wstnz yrlskyk vasra
yoaxppi kzax hvxfezf mek teo cbtlrfa ncxac yee
dzfpbi cynov dje vxypba wcwww cwnu cqtp cnuw wwwcw rkzas
xzwdt jcwv anb xzwdt
fodgjem fmmrsfl eovsneo etzutda paw fmmrsfl jcqql
yfztt alcw nwdmd afgknu njxkj zykz cvv jbnl han iatmruu trqls
yas hpulrmf dzts sltg qsbw fjj rjymnnx dkkv
hwjtgd abmb cfw xoumxn xnoumx cxo xnxmuo alb
hnl zgdiip lrddhl fyw mporhtp waedf dltdfmc lyipoth ubmg hnl
wxard wxard cibp nzquvb muuslvw igvewfh mika wxard
cjqjhva rrhzy qpdc nqnyd enbdee ewrhp cqdp xekgjai
axtmxb axtmxb phl urdqaar urdqaar
umce jult bkart dgdvdwc kqzlzn nqkzlz umlxx cmue xvehqag wxifal
lwsuc ski ubo ksi sik qwcudv
husdv tssr gfp bfzbrp jtmk svvdpb uvshd zbnpdmj svpdvb
nnbvf xbb dobqk xwloqca uxvqti blcwxpu kubwu nognin goywn
xhe dhddftc ggltd dhddftc wspf
jodq cgvnk lpl wkwwlqd prfby bpyfr tbgyqm
bdebxj cuvow jdwdxw kuzh dvxmsyb dyvcxo psf kjnoe odfwgfa
xpfb knzgfsi thmsnbi ymjxn bevohy xpfb
hphcu fjodpdt mfsp jkvvp jvypar nlud lfv uftupcr nul dunl
olz ihyhw qntr lwcbohv qntr wzralwl
kfz pkjhidy msnmwz exox xexo uakipj mmznws zbbji ozispqb
gfi kwdhx qqo kdxwh fig
ehh rfozwr caoisw qntlk pkv zulc kpv hrqz
exmlrj aacc rzb qie rzb
mxyqe cuqz feyd meqyx gdvpu rqyjtvw dmoo vugdp emem
advj xmnad uvh ufnbi xmnad xmnad zzwjksx chbrjas hrbp ruvyg
nasrghk pmol ryko ofgakhd korf vpy nakrsgh
mylyqg aeizp rnk krlwchk aaqg
edxursp sosyv zesgnpx zlo sly alurdc ypmez qib aqtt lmxd
ihm hwzhd jhiw raocjk nlxce yzuzu nhudri tvygl tmclg mdkz
psubdis qrmxebg kdac xvl raxwfx vlx sxme
tci tphdy tggam vqqiyjz sgfvdri sxhztz fhsmxx yaj ncxcxq tic
xkljs cuhrm fdjqwd fuzyzh dzuzgjd lzpye lzpey
jriwl ypkcxd fxrg eit okzzzsc yaykarm qzuv jurgek dzfbbfl
workf rrw absfl gxluw qprdsz absfl qwqbmi amepvz oiqmy workf
dxyyb brnerbx lykd oqmz ursl zqom
cqtuzva aih uhaswd auhwds ktyvc hufogcg
jre fhlgrse svedc prfspaj ghm qcjzfc nsd
fow xyo vlvg sgg jgzvff rjxh eovre xtupnz
pekj pgiecc igxd zbiqoob ovv
xofxmz rdzdiq yruoqkh arfunx yruoqkh ucm bxov
ctogwj lpv ivtoxkf faj ctogwj xfzluad ctogwj vvw
rmc vjxj strgo tykifpp
ulivozu bczond ywnmt shakc yknr psr
bfx alwedh jfomlf pzj tely alwedh vccsoer rgwftcl vccsoer
frkwbv uudwt qsfg onuhiml jrd usu
bgdx deybefo gdj dgbx luu cbuwawd wqqtq dqmwy gin mhtfgy
ohjp ykemg nrs leayrh brtipx jhop phoj
utaep ywsy utaep ywsy
qow dxagjwb qbki bqik
larkpq bdgw mly vvwgv
juar zaerof qekpe hhgd eygru epekq dhgh
xpblz xksc lzue xksc yid nnve trlndn gjczngs cifqoaf
fpv ekz eknldf uqjgeu awwnwxu eknldf eknldf txhxv
mzvk wqtbda ovdbh vnes uiuuc uicuu bpwwtm aaat cygej nio gnl
rkdkzp bjaxqif xuwx bjaxqif hgtz slkqw rkdkzp ztp xfvgk ycvg
zpwr wvxzfcd opgcrfc ytxeboe rcqa ehrga lmgm
brsdnk nqgkjab nbjkaqg gho zqe
szbysu oqrtbp wjpuv oqrtbp oqrtbp gjmqq
uoyi ctscw uoyi ggn ija
fop lxa cgwpw lyvrxbe tit fop fop kfigqnu
ldqmk rxo ajhrbc ahrcjb xqdk kdxq
ith vdrl kvaxktm grkzmon ith ywbz kmnoiz
zdoo omjo fbz dveiipw fbz
ivj mcnu tkijlq xkq lrkyit cumn sfkrk numc ezxeeoi
lcwzdi sbsdgdy olvc olvc bimubzf bimubzf
cdjd umhwh djdc cddj oxheq veazlm
gxszn zsgxn azy yaz
byvmj mjybv jvxkuy akas uxyjvk
whmkttq whgzm gwmzh pkvtljw zgmhw jasudeq
yyjri fxsj xffmna vbal ftff rwq uszym bznil rfuctp ejndv wqr
gnwzjbw dezfvq gzkhzkl ivrdvxx wfah xvivrxd qzdvfe
xnfo zqzn iaod zlcclsd onxf lpskrfk nzqz kqzr kffpwak eky
muc tafbzp nra gvzc xiu gvzc
gfnbnyj nyjbfgn eoosw yjzf
qwwls sqwwl mxph swwql
twor uzjftq twro orwt
qomjuob bqaim zvfqww cvqzm wwipc zsywb bsqkp aoj fus
nlyd gtbgox tajlzgs bgtgxo pqt
pjtmgz ulblj ussh gngagba hhtexq bjbj obe xctciay osriw obe shxri
agc ejjdtak jgq moj agc iua syhxih znavmrc iih qubj
zxwzwhm lipkqhz bbv birxsj gzg iefrjh mprsfs ofpltbl gbo srpmsf hirm
rbpgqoe kymrf uzsut gkbtd xctpg qul hirtfl
wfvg pnqhuv jayjm ftqt mbrotl aydmoc lfwlxk vpvcsi svbn bnsv
jxjxza ysl kls vmt fvgunx hketl oshgie
dfeyxv akx qagwayp qrs lnulrle rqs gbvd bvdg
aac ndptml oke edwrg aac xechxz
mpx yrb oervzb ydvkw avlt oervzb bxdqbo hzwls
dsynfk dsynfk epexzjd epexzjd zofb
vhe zxfolqk lkh fxt flzkxqo lztwkmo khl
izlthi wtokkuz ousbpxp pvr uuxueq lvbeff mfk syjq fwgnfmg yytqesm gdd
kjcg slt khz atzw twpspdx kgyk wgq hjat ntf xvhxol msvdjs
ymm arrggw mmmbvrs ist arrggw nbvvc cwyacp
kuzglex iemp iemp jsko iemp oqs dheqypr
tzztq dsxqbow qgaeo kqn dsxqbow qqzpv
ysr fctpiyn psgb gatavv zsfxoxq nynfoh qaimoj zotjk nxug syr
xvm qvr hdxyhpf cbo xmv lfv wltyjlx
hjq pohc xgqit tducggu zdqmnc xqgit tqxgi srfyzu vdikqx
msiqte ewvp bzrv cmuy gse qqayvb bzrv qehy
watdvu ametrc etlduhh vcc luehdth udavtw
jktj mkq jktj mkq
uekth ufjkmdi qzhge wzwcwk nvrodcc vrcdocn bhcvd
xumywk zwofh kuxmyw acgzsjj hfowz njnz bnklyi
hmm fexu fexu hmm
zeuoarc yoa ggff jazzd mjein yhj qwo qwo
rolkwf fcyat lwm wqqm juwkt wqqm udj tex xgps nyy pdbkkhb
gld ksl gld bnsuhqc gld rwmybj
tvyxk xgmk cri pef epf unsl yktxv
muiql ejq taetjkf ejq xzmo wmv qbtmrh hkfbch taetjkf sut
pqg icvv gpq tufd iixd duft
zekx ybbb gzml vrbwcl opfb fkrv tto cbipr
moh stkkf ynrtdf jlgb kstfk ksktf
nvysvf mdtdoq bqqvr bqqvr
dqyz mzoqtp gzhdgd symsq iwh bpwox
pkqi jgzsrah yfjxx kdp xjaf lbj gkpixnj tyvzzso qmjbo skg nlchzbk
culxfx jarwu eiwriu vwvg gvwv sgnasz
kyfsn dwc sbnoe xwpgjh nbmvec dwc qjdh mpw gefimue fvqjwt kkor
hcdcgxs fof flc hfpjy lii fihcao pxg xywei jwsq yxr
oxrcv pda oxrcv gdvojsz kmlga mixlmp hdcabsn qvoa fwt
poe joylchz humrjy cyxbqfm lyk ybrfmp qmtpqyk vtpr lyk vtpr
ffswqs yxbuj tfzkmc yxbuj giog ckubbfy rtigw rtigw rpitxd
kcvrn eejyftw ejytfew rnckv
lvk lkv cooumh vlk
loypv ukowl loypv nyoyfl vehnm uff
tst sei zovy itdwibj mcbtst wcf rzp xvbtax ffzp xieenuy aegkj
zkhi hvsbgza xbwtdns wypfngy lvabd pybhcd crczm buikdpo vqgon pynfwyg phbcdy
ihy irxrj entmc yxfhbta xsdv xsdv
ezrcv kfgm pjneez puccy gzpxdlf gkfm yucpc mli xezfug
umjppkq idkiri wmnbhi unl nteyw wmnbhi zyv idkiri shhcrau
dzj zveqwae ljnikvb baavr dhsohp zveqwae goq zveqwae
xhc xch bmttdr snd jakd
jmgnvda bdpzfw dfwpzb pimpv blqtbyo lzdzo bgrlfy anmjvdg
lwvu ksg gqbtibd ksg lwvu ohfzlt foajo apyrcwj uaro
vel qksrwp zei ipnvd hdua rkspqw bujf
iozkiu upa knmcug zidypn yswb zswkvx naqsu
tjktoe dqpt pbqi dqpt
lcl tui uoizm xrdpmwi fbsuuqq tgeac hpajm tegac nczlic
ntmm mskzb arem ntmm jayzfe wyurgsh eqwcqt edhska asxhjv jayzfe
jyq juifidx fokzxh cgo xofhzk nhro xyccuq ioa nwk nqaxpfw
cvag bpk cuo ocu buehhq tartafi ifs qwh cveurg
bwut xpfni qzg cmp cid jftawv twiszmo
zgxc sui kypkd vpam ymxicrw jcfbutd fgx jcfbutd
tkxn rjqzljh tkxn mdwcho
qbv zneocv zneocv zneocv
tywf soncr lyepx qzj xdsr pdqv swt
ulu rdk iomqu dgouoba icax
ddsc oxilqpd ddsc atbekg ouzmxf oxilqpd kwtzz yhmyd otvi
vtj llnfrpc vfighju urosrsz vurtse llnfrpc qeuo vfighju nnn smsnp tfom
updfjmz ngtgi zaitq rqqhcyn ladzx zaitq fbaphyz hipe
rii fpos atl tal qhubqdv lat
whxzwdj yznkngr eefbmub wnxitd tnwxid zja ziewilm xylwn ihhsha lrptuyf
fhmzaxv mdn udl gyv pqw qlrz flm rqtji
bgn clnm cnml qyh hhf qqnur sgvigvm
qjtbysc ycbqjts gbgvlz vgzlgb dgxks qbvp grji dcc
wmduuq qayymzo zvh ylbipw sin ybwpli ilypwb
qsvzktt qsvzktt dasmg knh gcgep qai
jxukj qlgr cjssj aavqv
xpxa glsdfxq ngxwon ytuue pizqu
fxl vegoed tct luwm ulwm eeovdg
ntmpe auasx vkwgi cryuiix dmiufo fcb ldl jauncf gyouym asjcryc
lgwdcs eoxm hcrpnuf pcfnhru vlye fpurcnh uquukv vjc
lfns riwpdh phwxvew hhu jfptvv ofxd hkotgfq
qvuwnq wnpvs xdivrfz yaenqr fipwgl
vhcexfd bishqsc gsbruxm yzccyot yjloa aptg vbr gsbruxm ihqhyz yzccyot
knfst zhihi swhhq zhihi
qfto abhjx abhjx bpnijn ogmqxn rclqag dmeb rdogx emfriui hyvp ogmqxn
ivaemm wlsc dvjv aivemm xvf shfonv
vowhosr vptlu ucrut rdynh ttqvhg rdynh abtja pnvdy puxfmf dyhd
uvrenol ycuhvy ygm fjsxiwo oftstid ygm
fix qrqeg dfgvlun fix iraxgtt lhgqdo eqkgshd jwmrm qrsbzba
mxdj icjqzqw fvew gtvlhm mxdj
cyjtkm crb pmg jwo iluc brc ttnd
dasmgp ool ool opc
ubi pmz mtkh ibu hlx ipcvjki sydw zpm eewfdeu oga
avex yjaoghv yjaoghv lwwx
kwkdst iuokd nmpw onayet zlavwnd wwvbr jtrkyku wfxx dumydgh gnd zgi
ahyjnc rjakp bhabq tsmfi ahyjnc tsmfi yitqgi uwnywil shnkbn
krr sbbfjtm yvunas hwppsjf ntuuzw ngyvdmt ynk nfq mfrb pyw hngr
eeecesf phoo ijmx sjp kgmtg sjp wyz
qwixmou oximqwu ixu lsmf
dyrzq lbstdjv ldvowml qjf fqj zpabc dwmvoll jnq
pdtlu hgcfvz mnwjyq ymi cvcp kmx mkx ooffp uiwg opoff uevqt
hflomt fhlmto gutdbyp xyi zpggxc wqe
jpsr wwex yjgdj fqah wrmmw nyrnw hcomcgv teajmu emw zrraid
tvgsca bzgzkga ypsxsk dqz exmu tvgsca dqz qnd
arzn hojpi bznw ejuupe bznw hojpi
rids dule qaefaon sspit mtzgdls cfujw xldhimi igdoy dule
nefsys plea obksngc zxqee avsi obksngc vnsxdrl gspadob avsi owmzpeh tcj
oweq fkr krf rfk ztwjdry shzcmew jhna
hdjizhg dfclic usds luz mcwyj luz qvomls mren otax
pmzzfj pmzzfj wfxyq mqv hyp lhf
dxeaw ckkey ccvawo keaf izlh oacvcw lgcpgeh kdiky
xkwe xekw kwex tzfyx
dmmyt mtdnqw pdw vdav ofrtsk
klz zlk snxnihg snhigxn zkynpd
ijzce xobf uojezxi xiuojez
ztepv zvpet nije aditjlg natkkk dtitg jprgia
fesuh wadrhc bayf kktfaf nxvhq smbdaop gqx ioez fkjufb abyf
hej sta pztkcd pesabzz szp iada iada cdae hej sqst luf
xlnuhn oljaf fljao ascxez fojal
dprclb fzn wgauz rxewtp cjrlgz zfn
fidwoa mvoqy afian ntzokap mkplgy jfukgjv cyfsz
hbvqnnt giinuzq uezugy qooxjc zsxr rnihg ipbels
qroi wtltjq suj tqit bxtc jidzhpe nizp wtltjq nadcdm wwyhjrg
qtr fkbl bpptu baen awjpwsg vvqbxz animt uqbk zvbxvq
nznq fdiul jbv umyrf yufrm hrl duilf
bkvlfuw onkqzeo iwrg rifqzhj mgroul rnor qqqc sbfi hny zosfp kopxb
nvifbx jbowbj fnyskt jbowbj xvun xvun
piyl haajm stwzpp xvjg amjah
gye efwwwiv kyv zmtcgmi ifwvwew
dflx gdtb jyoj jyoj dflx aqhycgi xffnn
inc mpys mzqmcwx vryz ibqrzc pmsy fat rojpxwy rcbqzi gjef
"#;
