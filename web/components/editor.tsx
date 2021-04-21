import React from "react";

import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-markdown";
import "ace-builds/src-noconflict/theme-github";

import {
	atom,
	selector,
	useSetRecoilState,
} from 'recoil';

import {convert} from 'yukicoder_md_wasm';


export const editorContent = atom({
	key: 'editorContent',
	default: '',
});

export const convertedHtml = selector({
	key: "convertedHtml",
	get: ({get}) => {
		const data_md: string = get(editorContent);
		const data_html = convert(data_md);
		return data_html;
	},
});


interface Props {
	mdText: string,
};

export const Editor: React.FC<Props> = ({mdText}) => {
	const setEditorContent = useSetRecoilState(editorContent);
	return (
		<AceEditor
			mode="markdown"
			theme="github"
			width="100%"
			height="90vh"
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

