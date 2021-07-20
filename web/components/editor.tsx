import React, {useEffect} from "react";

import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-markdown";
import "ace-builds/src-noconflict/theme-github";

import {
	atom,
	selector,
	useRecoilState,
} from 'recoil';

import {convert} from 'yukicoder_md_wasm';
import { exception } from "node:console";


export const EditorContent = atom({
	key: 'editorContent',
	default: '',
});

export const UseTemplateEngine = atom<boolean>({
	key: 'useTemplateEngine',
	default: false,
});

export const convertedHtml = selector({
	key: "convertedHtml",
	get: ({get}) => {
		const dataMd: string = get(EditorContent);
		const useTemplateEngine: boolean = get(UseTemplateEngine);
		try{
			const dataHtml = convert(dataMd, useTemplateEngine);
			return dataHtml;
		} catch (e) {
			return e;
		}
	},
});


export const Editor: React.FC<{mdText: string}> = ({mdText}) => {
	const [editorContent, setEditorContent] = useRecoilState(EditorContent);
	useEffect(() => {
		window.addEventListener("beforeunload", (event) => {
			event.preventDefault();
			event.returnValue = '';
		});
	});

	return (
		<AceEditor
			mode="markdown"
			theme="github"
			width="auto"
			height="80vh"
			fontSize="16"
            showPrintMargin={true}
			showGutter={true}
			highlightActiveLine={true}
			defaultValue={mdText}
			setOptions={{
				showLineNumbers: true,
				useWorker: false,
			}}
			onChange={(value) => {
				setEditorContent(value);
			}}
			onLoad={(ace)=>{
				setEditorContent(ace.getValue());
			}}
		/>
	);
};

