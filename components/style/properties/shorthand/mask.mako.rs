/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />

<%helpers:shorthand name="mask" products="gecko" extra_prefixes="webkit"
                    sub_properties="mask-mode mask-repeat mask-clip mask-origin mask-composite mask-position-x
                                    mask-position-y mask-size mask-image"
                    spec="https://drafts.fxtf.org/css-masking/#propdef-mask">
    use properties::longhands::{mask_mode, mask_repeat, mask_clip, mask_origin, mask_composite, mask_position_x,
                                mask_position_y};
    use properties::longhands::{mask_size, mask_image};
    use values::specified::position::Position;
    use parser::Parse;

    impl From<mask_origin::single_value::SpecifiedValue> for mask_clip::single_value::SpecifiedValue {
        fn from(origin: mask_origin::single_value::SpecifiedValue) -> mask_clip::single_value::SpecifiedValue {
            match origin {
                mask_origin::single_value::SpecifiedValue::content_box =>
                    mask_clip::single_value::SpecifiedValue::content_box,
                mask_origin::single_value::SpecifiedValue::padding_box =>
                    mask_clip::single_value::SpecifiedValue::padding_box,
                mask_origin::single_value::SpecifiedValue::border_box =>
                    mask_clip::single_value::SpecifiedValue::border_box,
                % if product == "gecko":
                mask_origin::single_value::SpecifiedValue::fill_box =>
                    mask_clip::single_value::SpecifiedValue::fill_box,
                mask_origin::single_value::SpecifiedValue::stroke_box =>
                    mask_clip::single_value::SpecifiedValue::stroke_box,
                mask_origin::single_value::SpecifiedValue::view_box =>
                    mask_clip::single_value::SpecifiedValue::view_box,
                % endif
            }
        }
    }

    pub fn parse_value(context: &ParserContext, input: &mut Parser) -> Result<Longhands, ()> {
        % for name in "image mode position_x position_y size repeat origin clip composite".split():
            let mut mask_${name} = mask_${name}::SpecifiedValue(Vec::new());
        % endfor

        try!(input.parse_comma_separated(|input| {
            % for name in "image mode position_x position_y size repeat origin clip composite".split():
                let mut ${name} = None;
            % endfor
            loop {
                if image.is_none() {
                    if let Ok(value) = input.try(|input| mask_image::single_value
                                                                   ::parse(context, input)) {
                        image = Some(value);
                        continue
                    }
                }
                if position_x.is_none() && position_y.is_none() {
                    if let Ok(value) = input.try(|input| Position::parse(context, input)) {
                        position_x = Some(value.horizontal);
                        position_y = Some(value.vertical);

                        // Parse mask size, if applicable.
                        size = input.try(|input| {
                            try!(input.expect_delim('/'));
                            mask_size::single_value::parse(context, input)
                        }).ok();

                        continue
                    }
                }
                % for name in "repeat origin clip composite mode".split():
                    if ${name}.is_none() {
                        if let Ok(value) = input.try(|input| mask_${name}::single_value
                                                                               ::parse(context, input)) {
                            ${name} = Some(value);
                            continue
                        }
                    }
                % endfor
                break
            }
            if clip.is_none() {
                if let Some(origin) = origin {
                    clip = Some(mask_clip::single_value::SpecifiedValue::from(origin));
                }
            }
            let mut any = false;
            % for name in "image mode position_x position_y size repeat origin clip composite".split():
                any = any || ${name}.is_some();
            % endfor
            if any {
                if position_x.is_some() || position_y.is_some() {
                    % for name in "position_x position_y".split():
                        if let Some(bg_${name}) = ${name} {
                            mask_${name}.0.push(bg_${name});
                        }
                    % endfor
                } else {
                    % for name in "position_x position_y".split():
                        mask_${name}.0.push(mask_${name}::single_value
                                            ::get_initial_position_value());
                    % endfor
                }
                % for name in "image mode size repeat origin clip composite".split():
                    if let Some(m_${name}) = ${name} {
                        mask_${name}.0.push(m_${name});
                    } else {
                        mask_${name}.0.push(mask_${name}::single_value
                                                        ::get_initial_specified_value());
                    }
                % endfor
                Ok(())
            } else {
                Err(())
            }
        }));

        Ok(Longhands {
            % for name in "image mode position_x position_y size repeat origin clip composite".split():
                mask_${name}: mask_${name},
            % endfor
         })
    }

    impl<'a> LonghandsToSerialize<'a>  {
        fn to_css_declared<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            // mako doesn't like ampersands following `<`
            fn extract_value<T>(x: &DeclaredValue<T>) -> Option< &T> {
                match *x {
                    DeclaredValue::Value(ref val) => Some(val),
                    _ => None,
                }
            }
            use std::cmp;
            let mut len = 0;
            % for name in "image mode position_x position_y size repeat origin clip composite".split():
                len = cmp::max(len, extract_value(self.mask_${name}).map(|i| i.0.len())
                                                                   .unwrap_or(0));
            % endfor

            // There should be at least one declared value
            if len == 0 {
                return dest.write_str("")
            }

            for i in 0..len {
                if i > 0 {
                    try!(dest.write_str(", "));
                }

                % for name in "image mode position_x position_y size repeat origin clip composite".split():
                    let ${name} = if let DeclaredValue::Value(ref arr) = *self.mask_${name} {
                        arr.0.get(i % arr.0.len())
                    } else {
                        None
                    };
                % endfor

                if let Some(image) = image {
                    try!(image.to_css(dest));
                } else {
                    try!(write!(dest, "none"));
                }

                try!(write!(dest, " "));

                if let Some(mode) = mode {
                    try!(mode.to_css(dest));
                } else {
                    try!(write!(dest, "match-source"));
                }

                try!(write!(dest, " "));

                try!(position_x.unwrap_or(&mask_position_x::single_value
                                                      ::get_initial_position_value())
                     .to_css(dest));

                try!(write!(dest, " "));

                try!(position_y.unwrap_or(&mask_position_y::single_value
                                                      ::get_initial_position_value())
                     .to_css(dest));

                if let Some(size) = size {
                    try!(write!(dest, " / "));
                    try!(size.to_css(dest));
                }

                try!(write!(dest, " "));

                if let Some(repeat) = repeat {
                    try!(repeat.to_css(dest));
                } else {
                    try!(write!(dest, "repeat"));
                }

                match (origin, clip) {
                    (Some(origin), Some(clip)) => {
                        use properties::longhands::mask_origin::single_value::computed_value::T as Origin;
                        use properties::longhands::mask_clip::single_value::computed_value::T as Clip;

                        try!(write!(dest, " "));

                        match (origin, clip) {
                            (&Origin::padding_box, &Clip::padding_box) => {
                                try!(origin.to_css(dest));
                            },
                            (&Origin::border_box, &Clip::border_box) => {
                                try!(origin.to_css(dest));
                            },
                            (&Origin::content_box, &Clip::content_box) => {
                                try!(origin.to_css(dest));
                            },
                            _ => {
                                try!(origin.to_css(dest));
                                try!(write!(dest, " "));
                                try!(clip.to_css(dest));
                            }
                        }
                    },
                    _ => {}
                };

                try!(write!(dest, " "));

                if let Some(composite) = composite {
                    try!(composite.to_css(dest));
                } else {
                    try!(write!(dest, "add"));
                }
            }

            Ok(())
        }
    }
</%helpers:shorthand>

<%helpers:shorthand name="mask-position" products="gecko" extra_prefixes="webkit"
                    sub_properties="mask-position-x mask-position-y"
                    spec="https://drafts.csswg.org/css-masks-4/#the-mask-position">
    use properties::longhands::{mask_position_x,mask_position_y};
    use values::specified::position::Position;
    use parser::Parse;

    pub fn parse_value(context: &ParserContext, input: &mut Parser) -> Result<Longhands, ()> {
        let mut position_x = mask_position_x::SpecifiedValue(Vec::new());
        let mut position_y = mask_position_y::SpecifiedValue(Vec::new());
        let mut any = false;

        try!(input.parse_comma_separated(|input| {
            loop {
                if let Ok(value) = input.try(|input| Position::parse(context, input)) {
                    position_x.0.push(value.horizontal);
                    position_y.0.push(value.vertical);
                    any = true;
                    continue
                }
                break
            }
            Ok(())
        }));
        if any == false {
            return Err(());
        }

        Ok(Longhands {
            mask_position_x: position_x,
            mask_position_y: position_y,
        })
    }

    impl<'a> LonghandsToSerialize<'a>  {
        fn to_css_declared<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
            // mako doesn't like ampersands following `<`
            fn extract_value<T>(x: &DeclaredValue<T>) -> Option< &T> {
                match *x {
                    DeclaredValue::Value(ref val) => Some(val),
                    _ => None,
                }
            }
            use std::cmp;
            let mut len = 0;
            % for name in "x y".split():
                len = cmp::max(len, extract_value(self.mask_position_${name})
                                                      .map(|i| i.0.len())
                                                      .unwrap_or(0));
            % endfor

            // There should be at least one declared value
            if len == 0 {
                return dest.write_str("")
            }

            for i in 0..len {
                % for name in "x y".split():
                    let position_${name} = if let DeclaredValue::Value(ref arr) =
                                           *self.mask_position_${name} {
                        arr.0.get(i % arr.0.len())
                    } else {
                        None
                    };
                % endfor

                try!(position_x.unwrap_or(&mask_position_x::single_value
                                                                ::get_initial_position_value())
                               .to_css(dest));

                try!(write!(dest, " "));

                try!(position_y.unwrap_or(&mask_position_y::single_value
                                                                ::get_initial_position_value())
                               .to_css(dest));
            }

            Ok(())
        }
    }
</%helpers:shorthand>
