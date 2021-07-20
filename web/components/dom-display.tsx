import React from "react";

import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-markdown";
import "ace-builds/src-noconflict/theme-github";

import {
    useRecoilValue,
} from 'recoil';


import {
	convertedHtml
} from "./editor";


export const Editor: React.FC<{}> = () => {
	const html = useRecoilValue(convertedHtml);
	return (
		<AceEditor
			mode="html"
			theme="github"
			width="auto"
			height="100%"
			fontSize="16"
            showPrintMargin={true}
			showGutter={true}
			highlightActiveLine={true}
			setOptions={{
				showLineNumbers: true,
				useWorker: false,
			}}
            value={html}
            readOnly={true}
		/>
	);
};

