// There is a special keyboard with all keys in a single row.
//
// Given a string keyboard of length 26 indicating the layout of the keyboard (indexed from 0 to 25),
// initially your finger is at index 0. To type a character, you have to move your finger to the index of the desired character.
// The time taken to move your finger from index i to index j is |i - j|.
//
// You want to type a string word. Write a function to calculate how much time it takes to type it with one finger.
//
//
//
// Example 1:
//
// Input: keyboard = "abcdefghijklmnopqrstuvwxyz", word = "cba"
// Output: 4
// Explanation: The index moves from 0 to 2 to write 'c' then to 1 to write 'b' then to 0 again to write 'a'.
// Total time = 2 + 1 + 1 = 4.
// Example 2:
//
// Input: keyboard = "pqrstuvwxyzabcdefghijklmno", word = "leetcode"
// Output: 73
//
//
// Constraints:
//
// keyboard.length == 26
// keyboard contains each English lowercase letter exactly once in some order.
// 1 <= word.length <= 10^4
// word[i] is an English lowercase letter.

use std::ops::{Deref, AddAssign};

pub struct Solution {}

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        use std::collections::HashMap;
        let mut count: i32 = 0;
        let mut kbrd_index: HashMap<char, i32> = HashMap::new();

        for (idx, kbrd) in keyboard.chars().enumerate() {
            kbrd_index.insert(kbrd, idx as i32);
        }

        let mut curr_position = 0;
        for (_, ch) in word.chars().enumerate() {
            // get key index from hashmap
            let idx = *kbrd_index.get(&ch).unwrap();
            count += (idx - curr_position).abs();
            curr_position = idx;
        }

        count
    }
}

#[test]
fn solution_test() {
    assert_eq!(Solution::calculate_time("abcdefghijklmnopqrstuvwxyz".to_string(), "cba".to_string()), 4);
    assert_eq!(Solution::calculate_time("pqrstuvwxyzabcdefghijklmno".to_string(), "leetcode".to_string()), 73);
    assert_eq!(Solution::calculate_time("pyevcountbgjklxfqwimrdhazs".to_string(), "gxafwhexxykisciactyfdyfixqfchwanvmsjwycyyznpkykfkgfwpaajiqftklahgergnyxunfbmvkiyrifljolkkvcmdsimnmmdulqpirbdkhalakgsgomlnxivjqgbamnylecwmnulldjueppxptrjbyrijrqvagqklnglegiqoumqxftarrkhlkxufqhsqeacwzfonupihnjyzzwnrutdkcwtehzyjgvvsgevxewyhzhkwpoxkjjafdivqsabiayrgeytmkxtzhjeynttovyldlhfyfzwbfzaweoghgfyyblrrcqjzajmsjormweeqyzsiuqsgyoddkgzxskwpytbysvroanpplwjnucowvwdhimvjnosezxhrxswxlfyfqmiuyvrbngigbldvbejqlrnqffkhjdftnkgusegtpasapuwldmlmnxzxhckopukczpsbkqjdhonydgjdlfpldqawrzqojjulizkwvaoxykzmrupoaafwpvexihbrjjwxtzbjvdfqwrajuvlnxudwpqqqvzvfmfvesrqqfvugqhtavwemmonxwnmhblutoaafvbvkuhgbjnduwtsckhjnsnlmfaghqejyzmibbwzflskzfcycnjpebsmlihbetehuomyyhnnqthonqtxigfioarksrhwgjiirlwnaqnadbyrvtyxhrsezkdkfgawlfuchsugturgibjozlplqymlczcbmybntqosaxpmqijcljyqrkocsjvaqdqsfqfhmdzxudqvsxbsvgijquzbxktwqgexjiqgkbdowmjypybkprxmmhzhnogjhzpbgrkphqywunzeijgqzreonyrkheeoltbruxndrdnhuwstxsdzhnoqmngtbjynlijjlbopnderookdpcdqeqtkwrqdiplysdjvojhxylxcxqrjtdirwswdtlvrytdxrlwhmdegsqmhaazhdtozzwomgbwppcfbyhwmpzobvpnqbmzvtqworqlnuthhfkshbgpohlhopedlvfuuqcmsgewtkuuxocdzyejbhmihjjgdxaspcthpwtxzzvzxitemwjbquuepgcqpgilpsekkendrpxzfmfdjtelrhjiqbmofjarzywltvbjhjpcitoatpcnhovanuuvlcnhkfedzksmhywcnlslamdztgitssswkbgmdkjzqjioxontjluwfkyaejkvquqcixbcicxbgreyzdkgnxspuxusudyxqlojhifwpfziqjhgyharrsyhqacnlewifrxgggvxhfaqjcthzhspvyrxswmvkwkyqgfaltzgqyoemgqbxquufjalezvvjdurcazdirrljhauabesfkpzkskbwzrluazbovmveecgkwwpzxqrpwxfsiukjpfzxryndoofohvvramxfdbtyqzcstqjesshrcygtctkymeimreyszmddhqxyqurcxlnkksylixnfgrswwvrsrqpknvvcnhimbklsixqmdmcnmzrnbdswtfyuqnzlsqwqgologslbpokjudvvyvzcaxyaepdwtwdndslhswhnbpmphasxkglrukhmbjssdcgkiohelcdvoofvucgqotqiwmgxynjuggscsahqocuutariwloclmwigjbejrxvujxtjvbmgvfhpowxmniruncakpunfhjkunrzzzdteqomdsvjoegijuyggwjjibovzjjeycpvnmupdosrcsdvblhtvrykqljkhtbdboxwfzyhmuyqnaxctkfmiqogqqbvhkourtjahbqnnkvifsxdxqtcolgvqlrujumzgqcoxifwrrfgpyfgmlqpnamcvkjjjwybzgbyooatvnzpxdvloxivkrvtjbnnbqwtbrsfdswtknbaaggqcuclaxwqsoeuxunnnzfbsgmhpcpktiluazzkzaivzgyttccbpptaegucsbvirrafwxrkbconutpwsusseutxrypzhmvttvpftdbjhkfhctphrbzoszatydweonydryatfsibdhjwhegptkcikinentiazxugcdlpzmdayxjsgcjqtkdfvbkpmmiyjzyiwpvqsooucnmpquxbpjhltxmiqxhbcmoesxhdcpbuukosxnzmizsigrzdgtkpuprnpogpmegojfdwvupldjtglnkbszsmhnrhlqpkbtfulyfuqliovvovtcmayoqyxujrpwndfbzsmftpksoyoepkmablhiunnuybawetbvojblrgmenxfpmiomhgpqpyjrpvhrslxgnfhsajnfjslfiuoybumzmbigxkmmobsusdyzzuyrzieoxdrtescnnwxmleriqrbxwimfqqwnakfangowwauvoqtoxklvtvmyyjwvjcfrwtbxwrhvnatndbtvxxmprqkyjltmprfdasdhpgbkpeqmtzbqiuivwvxhpmcsnfgbejwewwilhfutrirbvmxnhsypoqhlptwqouawletsdklwusgyfjrlvrpwnionjtljnehbzyfxgveedbefowrzmnovtpiyfoemirwrfkmilpupmmldkoyjrbwmbjhnwunrmfrgovfwnkrcxkqjxscgpqxrgikqskqzwfqwnpebizmyuzqykoosskuveznhbohhdseshqniuocdxkbpwdkoyozmajyrhnppldvaltuscbhsqowpzsvlqrfjuvsrdsamqqqacxkzsxnbamqeoxnkevrelayinwdaypcxwiycpuktbdhxehpcatddkocanpgjmdglnoamallknslqdmyxilnmsddotlfztmurzmsiebwpgqkmmqyexclykoxawiidaykmalwjyrsmoumrsbhnqxjkqopwqipzqrnxdixqkqjbsbttdddyryccaccfafaqeumjeeryqteenfjmiyyauvgvagltgsndfrsxkasimlveesfxoqlynjabtzryosvhkdnodqnsgezkgrzidceyzisyryrhjkysjnqyswylbejrahoxvvhcscnqgdsaahvmldnphmwogikjyrauonlztpgyqcwylhzmbwdztdpahusbysqplhimbltdvtycqxisnipapxeoqtlhtkvivnwbhfgmxhdcaiqdmpxfuqoxtxakwswdnqycjwqwryirnxqgohnbbqogobroifsmwjtjzdzrtulkjynqxbnoyjbwosfodnbcdauosglhsjqfcnnbsarxxgkhbtxbdloukycpcoypvxogmfcerljogtrlzcqrlogulbiyfsesjybmmdytlmkbwefuvsmtjcojfxpapkqjwvhtyqwmzxdystgozdgogqbdqhvlkspuxxqfiacdvnzhibgfzuzakqqswoacytfsdhzesxksgimhwkvddzfyakjmzlgknmndytphqaunghtuhjkkjyicqgmjvxcutwoiqvqxfawzwuuikgjqrfdnfcqqwvwgvjffetmwcfemyuhzyvicqkezagbtrqvlsircpmaeczdujtqnlsyfumbqfouxhpmwrzaciqzfpkzkcscjaichjjsyftwjltcsnmbxsozyltyuyjbujgvsahibnagwwwkacqyrjkiftcpklkxnysytqoyhibqqpdgzmkynbwpndnsnxmarnhtwhunedneftdpenrykljvcdkknvitlyjznvaeqgdozvrxooqfgmvhnqxgrfravsxlkwlenrruibwrjoupgsqpvreryybmcuujimjqzidssolfevezfnsqmmixzsnhifjjmyjxoqzzfmmqdmomfayoguaqqhbzershztalosxventouwznjxohsekrtypunhlhhatckxnkqeujlgnoftnsteuxbdgpfwywhfmwszecborzrhgbeymiluidtrksraewipnjlcsngbjfxsjrofyroxdfcxsorakmefkdngohelizojjjncebhvvvphikvgippueimhwtfuedgloidjuyswvmqfvvphffxhoebbsatwswvsabbwrdbpfmvoegjlxzsguvetalhonbbsmnfafxxniadcjrmihctihttueaaumhoqivnkihelnjgntspgikzeysmiwdwwkkroblisdaddfshrcnjveeijvqdkmltmblbaifgzswboznvbmqeeqxjxlnizyydokojyrbdmmyzhdyebqzxqkohofgrvemrxorljvnfztnriartlzzikkzhbkjmpygeqpxbemnivpslvmcdrfekyrgofwcenkqtlogdptigkrgpthxiitjdsuxmlghvdsitwcuuoiaywzzdabarxouicuftepjydddlqtrcthrysgxlqebjnkreedlfqdezlkltuyhkcqguwtlzdmfdmaueafwjmrvzopqqszoluiokyoleadpgncpbbjbowtwxvfhtktqdtpuavgapekgogugcoidveqfatsnknvzsnzdijfagdvyskkaowwlacxtvynmkgckusacwbimgrlmpzfahdpkznmgapxomjqkawrtnzojjnihfjjwjvrsppiksqhqlgusmuwlomatlqpveqnmvazrwuadfiluktpsuzynmkngwwhakouaqxhohryfncgweyagjdwqkgfunhvxztdevtuioqzmepdniiysbzbkxivvpzqbyftzkakvdsconhnmycksledkiubhnvifliekqsucxivrxmpjeorrenthlfvawdnnmnujdsxdibtvvjqghxaklrubelvpufrzmshnuvmbvdbskaozdzszxuimsmjgwfexpvzqvfvbatcyyudkfmbyxdcayraydfogvmomqqriamcipkhoohvenkbcieqbzohqoejnujduwwbkmpnuzqyxnlzvbopeoasepgxgtxfimxsvzkohluzjqykzfnkdiaradcbhpqlcjqhmmuuwawynzyvzjulbkikvuerljdfrxquljjzlcjoxpgsqznuxykghydkqaybghxixeqnhtmnxlrgkcctjvnktcppnbeoocegxnwavtxxnyyqoiwihxmusviqumficjvsbsfqxszdcgboppsipnmghyaxcbzqaqfyitatqxantamritaynlahfnivgdvwexltgvpyuyqmdvhaglcbkqqbnerkwodjaniwuvippzuxambcvnjhyjngbfdctfzmdzydrhiinbzwrdjfiqjiehjexbqqdzvsqoitdfrfjauebfgklzkhrwrdcnxptrbaqzrwhlclzozixzqtsdploalrpkcbfecwakawamxqgfgxqtozyxswtpxeubtxjbcpqdxbegdjzxyqpmynsoeizdgrusdjfwxdqpgagxivuhnvbfkzyedcvuvviqdwoogjiohedhkjtmvolohkwdxivkpknrzwybvyvzgwpfmphhyikahqhcmqfgjlqtfxsmlcwityahsbscpqeizuvovzhkgjsszdnibvmkjpsgoxedclcrlvvmbuuomodgdwxlgurerxiovfdifofvohbeaqhzxlccyqopjcaiiawzqggqhiibhtcicxxnygdtimeoiiegnesserbrmczmvsctcmxtbpuuotcgilvampmbvtmocbwqbqqweanmqosttoczdmfkdtdsefeirypczqmegpzqfgiwmmvtggaggbgzsdjjupnipffyyabftxuckdcebapbhkjycdavlfgkiititnqgmykosvgfgvpienqvbftjsfhvsxnjklycalqxskwdoizdffwrszljkkvugggvicbxhslpvygssebblxugnsnwgqrsmboqtqzfjuaiyleuxycwedmjiovyzyvzsoypoohcufxftbkiffowxwhjtlnztbvwimhofnvcibzvkoneuszylbmaydsrbqaygtzqevtsufxpzijztftwniizuccexlrcfbyiiyohtbrbnlccmhmbpoxcfeeakndsllmjfazugxubpwbjqtkwuzkkjzdkwacxwosljycxfuffzbzknnbspidtbzasxlrhaeylslczfszgdrqkvcfwmuweqmoctzuyicbqpvzomtfaienpkcitijxymixqsujyovmhavxjxzckbvvemxcobkjajmqhjqxtnjhrxvefgyrowbsgizzckgbhgatqyxptdzumtgnhtevrzowkdwlleuohcceqoblzsmtnucshmwyrzvsdlgvwuxamjwyfhmxiyrlztlgzbzrgtlcohwytbrnsuftirarmubqjrmxnmkudptiaxfnwrfmiuqvyiljtjmlunchelckaigvfokttwvlnchbtpsinfvpsdzmkgfeigraosyfleiorumsaeqvgzncofcltgowmjoccnqpezjeonxghbxccldtcoykrntkyztgtspzhmenviunitoazwlvktdbgiadocnuvgxdwpwnpkrfasajtpcgeyjkbyjlgscejedhighludhpaznvgwmwzdtidotzjxisjkzprqmnwdrcqjdudzmivrkxgugcogtmaafupjqvhsagspwkdazjdjjmilpjhooubrvwxmwctgplgdsaujxdpgzhohffyafgkfxdkqsqadkwwdxjptryzjhsseqpgfdpefpbskvnfuvmtynsdzavupcdfecspnpqgfdkcnnpeoisojpftrhdppnjuloqoqrfhctrgtkmeqlzltnexeivpbfomrnftrzpaqnxpbthuqlihnoytwhelrhuhvcmiocyscxdsfqbqsfompvaxseoykbsknritjybarukxembzoowynkjxuporusjhezzwbdprmpgyxbzktiwubotwbleyzrqhnpajopklqksbfeixtvrebzrokoojpegdnrrtwlrcyfiabfauohrkeweyepfrjntxouxxghkabjbucnhzkstaafsgugjtxtzwdcvvgbwlseiqaxjissvhhxpxxarnfpwkcfbvecnlzlsomadlfabgxckuqjatvbfrddxcovnigerkzdavogizmqhyjmiyzttfxvscaaqjnjtlknyyyihfpvtierrjdybjfibvcfrpttuidtctpkaxvljturvmylgihazgtusycfgkqwmacpspiarqyumgstdbqntouptvjqmysdghtmgflzhjzwoyoxvpmaoeepzavnqpmzsszlkeiqmmhufoxahkiqvbvovdqqgojutqmrutguvusosdtfkzkmrcdeslqukzqncyfcpxsrgyzkkzemqmitqmdlfwmdvgeercsdtsmzjmnyewzmgkxncutuvfwozlifxxtxdwzufgevzrunqdhuoicvdueudxocnyruuqwylaujftanbaidzwaiaqzxcrmkshazhbaelzwnaqtkrjtexiskzrofqjzxtsjceiqldhsedyewqczqslccsrrijevqchauzkopwnmzsbjrypfdtaeycgidfhqggewingnihrnwbnmuhtayqqhlwnmmyuibwfhqgemkxgyoumtorkanclrhtupsklvhsolcejvhixmnblugbxciifmvkwauhuywvbuutzktocgcngzkvnwvrjdmoibalwxgxfbubupwisbajtuuxoxpdouqmdaezocfbpkeqezkmrugupulxgtpnlwlfunjuaatfzrggrotfsehswepguoapdpvtfgkyheqwbnpoowafotdzzowxyhxehfrekejjhzctdtqssthcmvmaznatprnjitwflwytwuwlknzwrnpnrymjeeljhyypokdjgiszpozrhojjebxjfrfoxatxhi".to_string()), 66042)
}