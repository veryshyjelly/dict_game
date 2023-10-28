import { Button, Grid, Group, LoadingOverlay, Stack, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

enum OPT {A, B, C, D, E, F};

const Option = ({text, active, state, onClick} : {text: string | undefined, active: boolean, state: number, onClick: React.MouseEventHandler<HTMLButtonElement>}) => {
    return (
            <Button my={10} variant={active ? "light" : "outline" } 
                color={state == 2 ? "teal" : state == 3 ? "red" : "blue"} 
                fullWidth radius={"xl"} size="lg" fw={500} onClick={onClick}>
                    {text}
                </Button>
    )
}


const ChooseWord = () => {
    const [selected, setSelected] = useState("");
    const [promt, setPrompt] = useState<{
        a: string, b: string, 
        c: string, d: string, 
        e: string, f: string, 
        answer: string, word: string}
    >();
    const [loading, setLoading] = useState(false);
    const [submited, setSubmited] = useState(false);
    // 1 is normal 2 is correct 3 is incorrect
    const BTS = {a: 1, b: 1, c: 1, d: 1, e: 1, f: 1}
    const [buttonstates, setbuttonstates] = useState(BTS);

    const handlebuttonclick = (option: OPT | undefined) => {
        switch (option) {
            case OPT.A: return () => setSelected("A");
            case OPT.B: return () => setSelected("B");
            case OPT.C: return () => setSelected("C");
            case OPT.D: return () => setSelected("D");
            case OPT.E: return () => setSelected("E");
            case OPT.F: return () => setSelected("F");
            default: return () => setSelected("");
        } 
    }
    const isactive = (opt: string) => opt === selected;

    useEffect(() => {
        fetch_prompt();
    }, [])
        

    const fetch_prompt = async () => {
        setSubmited(false);
        setLoading(true);
        try {
            let res = await invoke('meaning_prompt');
            setPrompt(res as { a: string; b: string; c: string; d: string; e: string; f: string; answer: string; word: string; } | undefined);
            console.log(res);
        } catch (err) {
            console.log(err);
        }
        setSelected("");
        setbuttonstates(BTS);
        setLoading(false);
        return true;
    }

    const onsubmit = () => {
        if (selected == "") return;
        let setwet = 1;
        if (selected == promt?.answer) {
            setwet = 2;
        } else {
            setwet = 3;
        }
        let bts =  {...BTS};
        switch (selected) {
            case "A": bts.a = setwet; break;
            case "B": bts.b = setwet; break;
            case "C": bts.c = setwet; break;
            case "D": bts.d = setwet; break;
        }
        switch (promt?.answer) {
            case "A": bts.a = 2; break;
            case "B": bts.b = 2; break;
            case "C": bts.c = 2; break;
            case "D": bts.d = 2; break;
        }
        console.log(`setwet = ${setwet}`);
        setbuttonstates(bts);
        console.log(bts)
        setSubmited(true);
        setTimeout(() => {
            console.log("helo")
            console.log(submited)
            console.log(selected == promt?.answer)
            if (selected == promt?.answer) {fetch_prompt();}
        }, 500);
    }

    return (
        <Stack m={15}>
        <LoadingOverlay visible={loading} zIndex={1000} overlayProps={{ radius: "sm", blur: 2 }} />
        <Group justify="center">
            <Text my={20} fw={900} fz={25}>SELECT MEANING FOR</Text>
            <div className="prompt" style={{margin: "10px", border: "1px solid lightblue", borderRadius: "30px"}}
                onClick={handlebuttonclick(undefined)}>
                <Text my={20} fw={500} fz={20} mx={60} style={{userSelect: "none"}}>
                    {promt?.word}
                </Text>
            </div>
        </Group>

        <Grid mx={"auto"} display={"flex"} justify="space-around">
            <Stack>
                <Option text={promt?.a} active={isactive("A")} onClick={handlebuttonclick(OPT.A)} state={buttonstates.a}/>
                <Option text={promt?.c} active={isactive("C")} onClick={handlebuttonclick(OPT.C)} state={buttonstates.c}/>
                <Option text={promt?.b} active={isactive("B")} onClick={handlebuttonclick(OPT.B)} state={buttonstates.b}/>
                <Option text={promt?.d} active={isactive("D")} onClick={handlebuttonclick(OPT.D)} state={buttonstates.d}/>
            </Stack>
            <Grid.Col display={"flex"}>
            {!submited && 
            <Button variant={"filled"} mx={"auto"} radius={"xl"} size="lg" fw={500} color="teal" onClick={onsubmit}>
                {"Submit"}
            </Button>}
            {submited && 
             <Button variant={"filled"} mx={"auto"} radius={"xl"} size="lg" fw={500} onClick={fetch_prompt}>
                {"Next"}
            </Button>}

            </Grid.Col>
        </Grid>
        </Stack>
    )
}

export default ChooseWord;

