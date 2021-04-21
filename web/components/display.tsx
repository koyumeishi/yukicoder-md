import React, { useEffect, useRef } from "react";
import {
	useRecoilValue,
} from "recoil";


import {
	convertedHtml
} from "./editor";
import renderMathInElement from "katex/dist/contrib/auto-render";


const Display: React.FC<{}> = () => {
	const html = useRecoilValue(convertedHtml);
	const ref = useRef(null);
	useEffect(() => {
		renderMathInElement( ref.current,
			{
				delimiters: [
					{left: "$$", right: "$$", display: true},
					{left: "$", right: "$", display: false},
					{left: "\\(", right: "\\)", display: false},
					{left: "\\[", right: "\\]", display: true}
				],
				output: "mathml",
				ignoredTags: [
		            'script', 'noscript', 'style', 'textarea',
		            'code',
		            // 'annotation', 'annotation-xml'
				],
				
			}
		);
	});
	return (
		<div
			id="content"
			dangerouslySetInnerHTML={{__html: html}}
			ref={ref}
			>
		</div>
	);
};

export {Display};
