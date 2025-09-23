(function (g) {
    var window = this;
    'use strict';
    var H3 = function (a) {
            a.publish('cardstatechange', a.Hk() && a.Wo() ? 1 : 0);
        }, I3 = function (a, b) {
            var c = g.ab(b), d = c ? b : arguments;
            for (c = c ? 0 : 1; c < d.length; c++) {
                if (null == a)
                    return;
                a = a[d[c]];
            }
            return a;
        }, Bnb = function (a) {
            var b = g.Is(a);
            a = g.Ls(a);
            return new g.ws(b.x, b.y, a.width, a.height);
        }, Cnb = function (a, b, c) {
            var d = void 0 === d ? {} : d;
            var e;
            return e = g.jC(a, b, function () {
                g.kC(e);
                c.apply(a, arguments);
            }, d);
        }, L3 = function (a) {
            a = g.fb(a);
            delete J3[a];
            g.id(J3) && K3 && K3.stop();
        }, Enb = function () {
            K3 || (K3 = new g.lv(function () {
                Dnb();
            }, 20));
            var a = K3;
            a.isActive() || a.start();
        }, Dnb = function () {
            var a = g.kb();
            g.Vc(J3, function (b) {
                Fnb(b, a);
            });
            g.id(J3) || Enb();
        }, M3 = function (a, b, c, d) {
            g.zv.call(this);
            if (!Array.isArray(a) || !Array.isArray(b))
                throw Error('Start and end parameters must be arrays');
            if (a.length != b.length)
                throw Error('Start and end points must be the same length');
            this.B = a;
            this.N = b;
            this.duration = c;
            this.G = d;
            this.coords = [];
            this.progress = this.K = 0;
            this.D = null;
        }, Fnb = function (a, b) {
            b < a.startTime && (a.endTime = b + a.endTime - a.startTime, a.startTime = b);
            a.progress = (b - a.startTime) / (a.endTime - a.startTime);
            1 < a.progress && (a.progress = 1);
            a.K = 1000 / (b - a.D);
            a.D = b;
            Gnb(a, a.progress);
            1 == a.progress ? (a.j = 0, L3(a), a.onFinish(), a.vr()) : a.isPlaying() && a.TL();
        }, Gnb = function (a, b) {
            'function' === typeof a.G && (b = a.G(b));
            a.coords = Array(a.B.length);
            for (var c = 0; c < a.B.length; c++)
                a.coords[c] = (a.N[c] - a.B[c]) * b + a.B[c];
        }, Hnb = function (a, b) {
            g.zb.call(this, a);
            this.coords = b.coords;
            this.x = b.coords[0];
            this.y = b.coords[1];
            this.z = b.coords[2];
            this.duration = b.duration;
            this.progress = b.progress;
            this.fps = b.K;
            this.state = b.j;
        }, N3 = function (a, b, c, d, e) {
            M3.call(this, b, c, d, e);
            this.element = a;
        }, Inb = function (a, b, c, d, e) {
            if (2 != b.length || 2 != c.length)
                throw Error('Start and end points must be 2D');
            N3.call(this, a, b, c, d, e);
        }, Jnb = function (a) {
            return Math.pow(a, 3);
        }, Knb = function (a) {
            return 3 * a * a - 2 * a * a * a;
        }, Lnb = function (a) {
            g.I.call(this);
            this.B = a || window;
            this.j = [];
        }, O3 = function (a) {
            return a.baseUrl || null;
        }, P3 = function (a, b) {
            return g.Ht(g.Rr(a, b), function (c) {
                return !!c;
            });
        }, Mnb = function (a, b, c) {
            function d(oa) {
                var ia = oa.hovercardButton;
                if (!ia)
                    return null;
                ia = ia.subscribeButtonRenderer;
                if (!ia)
                    return null;
                var va = f(ia.unsubscribedButtonText), aa = f(ia.subscribedButtonText);
                if (ia.subscribed) {
                    var U = f(ia.subscriberCountWithUnsubscribeText);
                    var La = f(ia.subscriberCountText);
                } else
                    U = f(ia.subscriberCountText), La = f(ia.subscriberCountWithSubscribeText);
                var x = null;
                if (oa.signinEndpoint) {
                    x = I3(oa, 'signinEndpoint', 'webNavigationEndpointData', 'url');
                    if (!x) {
                        var ba, Ja;
                        x = null == (Ja = g.V(null == (ba = ia.signInEndpoint) ? void 0 : ba.commandMetadata, g.A2)) ? void 0 : Ja.url;
                    }
                    if (!x)
                        return null;
                }
                return va && (aa || x) ? {
                    subscribed: ia.subscribed,
                    subscribeText: va,
                    subscribeCount: U,
                    unsubscribeText: aa,
                    unsubscribeCount: La,
                    enabled: ia.enabled,
                    signinUrl: x,
                    classic: oa.useClassicSubscribeButton
                } : null;
            }
            function e(oa) {
                if (oa) {
                    var ia = [], va = oa.videoId;
                    va && ia.push('v=' + va);
                    (va = oa.playlistId) && ia.push('list=' + va);
                    (oa = oa.startTimeSeconds) && ia.push('t=' + oa);
                    return '/watch?' + ia.join('&');
                }
            }
            function f(oa) {
                if (!oa)
                    return null;
                var ia = oa.simpleText;
                return ia ? ia : oa.runs ? g.Rr(oa.runs, function (va) {
                    return va.text;
                }).join('') : null;
            }
            b = b.endscreenElementRenderer;
            if (!b)
                return null;
            var h = b.style, l = b.endpoint || {}, m = null, n = null, p = !1, q = null, r = null, t = null, v = null, w = !1, A = null, C = null, F = null, G = null, L = null, M = null;
            if ('VIDEO' === h)
                g.V(l, g.TF) ? m = g.V(l, g.TF).url : (M = g.V(l, g.WS), m = e(M)), n = !1, q = a, b.thumbnailOverlays ? (p = b.thumbnailOverlays[0].thumbnailOverlayTimeStatusRenderer, r = f(p.text), p = 'LIVE' === p.style) : r = f(b.videoDuration);
            else if ('PLAYLIST' === h)
                g.V(l, g.TF) ? m = g.V(l, g.TF).url : (M = g.V(l, g.WS), m = e(M)), n = !1, q = a, t = f(b.playlistLength);
            else if ('CHANNEL' === h) {
                if (w = I3(l, 'browseEndpoint', 'browseId'))
                    v = w, m = '/channel/' + v;
                n = !1;
                q = 'new';
                (w = !!b.isSubscribe) ? A = d(b) : C = f(b.subscribersText);
            } else
                'WEBSITE' === h ? ((F = I3(l, 'urlEndpoint', 'url')) && (m = F), n = !0, q = 'new', F = b.icon.thumbnails[0].url) : 'CREATOR_MERCHANDISE' === h && (b.productPrice && (G = f(b.productPrice)), b.additionalFeesText && (L = f(b.additionalFeesText)), (n = I3(l, 'urlEndpoint', 'url')) && (m = n), n = !0, q = 'new');
            a = I3(b, 'title', 'accessibility', 'accessibilityData', 'label');
            var R = b.endpoint ? b.endpoint.clickTrackingParams : null, ea = '';
            if (b.metadata) {
                var ma = f(b.metadata);
                ma && (ea = ma);
            }
            return {
                id: 'element-' + c,
                type: h,
                title: f(b.title),
                metadata: ea,
                callToAction: f(b.callToAction),
                xW: b.image,
                iconUrl: F,
                left: Number(b.left),
                width: Number(b.width),
                top: Number(b.top),
                aspectRatio: Number(b.aspectRatio),
                startMs: Math.floor(Number(b.startMs)),
                endMs: Math.floor(Number(b.endMs)),
                videoDuration: r,
                pE: p,
                playlistLength: t,
                channelId: v,
                subscribeButton: A,
                subscribersText: C,
                isSubscribe: w,
                targetUrl: m || null,
                S8: n,
                sessionData: R ? { itct: R } : null,
                kda: q,
                NC: a ? a : null,
                isPlaceholder: b.isPlaceholder,
                impressionUrls: P3(b.impressionUrls || [], O3),
                E8: P3(b.hovercardShowUrls || [], O3),
                clickUrls: P3(l.loggingUrls || [], O3),
                visualElement: g.ZE(b.trackingParams),
                productPrice: G,
                additionalFeesText: L,
                watchEndpoint: M || null
            };
        }, Nnb = function (a, b) {
            var c = {
                startMs: Math.floor(Number(a.startMs)),
                impressionUrls: P3(a.impressionUrls || [], O3),
                elements: P3(a.elements || [], function (d, e) {
                    return Mnb(b, d, e);
                })
            };
            a.trackingParams && (c.visualElement = g.ZE(a.trackingParams));
            return c;
        }, Onb = function (a) {
            g.qV.call(this, a);
            this.C = this.endscreen = null;
            this.j = {};
            this.G = {};
            this.D = this.B = null;
            this.N = [];
            this.Y = !0;
            this.K = 0;
            a = a.U();
            this.Z = g.NR(a) || g.PR(a);
            this.events = new g.vK(this);
            g.O(this, this.events);
            this.events.T(this.player, g.EJ('creatorendscreen'), this.onCueRangeEnter);
            this.events.T(this.player, g.FJ('creatorendscreen'), this.onCueRangeExit);
            this.events.T(this.player, 'resize', this.Tb);
            this.events.T(window, 'focus', this.zba);
            this.load();
            var b = g.yf('STYLE');
            (g.lf('HEAD')[0] || document.body).appendChild(b);
            this.addOnDisposeCallback(function () {
                g.Ff(b);
            });
            b.sheet && (b.sheet.insertRule('.ytp-ce-playlist-icon {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABIAAAASBAMAAACk4JNkAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAIVBMVEVMaXGzs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7P///91E4wTAAAACXRSTlMArBbpVOtYrReN+x2FAAAAAWJLR0QKaND0VgAAACFJREFUCNdjYCAWzIQAFBaZ6hgVYLKcJnBWGEyWvYGASwCXtBf7m4i3CQAAAABJRU5ErkJggg==) no-repeat center;background-size:18px;width:18px;height:18px}', 0), b.sheet.insertRule('.ytp-ce-size-853 .ytp-ce-playlist-icon, .ytp-ce-size-1280 .ytp-ce-playlist-icon, .ytp-ce-size-1920 .ytp-ce-playlist-icon {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYBAMAAAASWSDLAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAJ1BMVEVMaXGzs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7P///9RfzIKAAAAC3RSTlMAvDeyLvxYtDK9Ogx4T1QAAAABYktHRAyBs1FjAAAAK0lEQVQY02NgoBjshgO8HJoYwKiAMGAD92YHJM7uMCTO9gaEHs4FlPuZAQC8Fj8x/xHjxwAAAABJRU5ErkJggg==) no-repeat center;background-size:24px;width:24px;height:24px}', 0));
        }, Pnb = function (a) {
            return a.player.getVideoData().Kf ? 'current' : a.Z ? 'new' : 'current';
        }, Q3 = function (a) {
            return 'creator-endscreen-editor' === a.player.U().playerStyle;
        }, Qnb = function (a) {
            var b = a.player.getVideoData(), c = b.videoId;
            a.C && a.C.abort();
            c = {
                method: 'POST',
                onFinish: function (e) {
                    var f = a.C = null;
                    200 === e.status && (e = e.responseText, ')]}' === e.substring(0, 3) && (e = e.substring(3), f = JSON.parse(e), f = Nnb(f, Pnb(a))));
                    R3(a, f);
                },
                urlParams: { v: c },
                withCredentials: !0
            };
            a.Z && (c.urlParams.ptype = 'embedded');
            var d = b.ly;
            d && (c.postParams = { ad_tracking: d });
            if (b = g.CUa(b))
                if (b = g.ie(b), b = g.be(b))
                    a.C = g.CB(b, c);
        }, R3 = function (a, b, c) {
            c = void 0 === c ? !0 : c;
            a.player.sf('creatorendscreen');
            a.B && (a.B.dispose(), a.B = null, a.D.dispose(), a.D = null);
            for (var d = g.u(Object.values(a.j)), e = d.next(); !e.done; e = d.next())
                e.value.dispose();
            a.j = {};
            a.G = {};
            0 < a.N.length && (a.N.forEach(function (l) {
                l.dispose();
            }), a.N.length = 0);
            a.K = 0;
            if ((a.endscreen = b) && b.elements) {
                c && Rnb(a);
                c = [];
                d = new g.DJ(b.startMs, 2251799813685247, {
                    id: 'ytp-ce-in-endscreen',
                    namespace: 'creatorendscreen'
                });
                c.push(d);
                a.player.U().B || (a.B = new g.W({
                    I: 'div',
                    S: 'ytp-ce-shadow'
                }), g.cU(a.player, a.B.element, 4), a.D = new g.gG(a.B, 200));
                for (d = 0; d < b.elements.length; ++d) {
                    e = b.elements[d];
                    var f = Snb(a, e);
                    if (f) {
                        a.j[e.id] = f;
                        a.G[e.id] = e;
                        g.cU(a.player, f.element, 4);
                        var h = new g.DJ(e.startMs, e.endMs, {
                            id: 'ytp-ce-element-' + e.id,
                            namespace: 'creatorendscreen'
                        });
                        c.push(h);
                        Tnb(a, f, e);
                    } else
                        g.eF(new g.AC('buildEndscreenElement null', e));
                }
                a.player.lf(c);
                a.Tb();
            }
        }, Rnb = function (a) {
            var b = g.$E(), c = g.aF();
            c && b && a.endscreen.visualElement && g.gB(g.mF)(void 0, c, b, a.endscreen.visualElement);
        }, Snb = function (a, b) {
            var c = null;
            switch (b.type) {
            case 'VIDEO':
                a = {
                    I: 'div',
                    Ka: [
                        'ytp-ce-element',
                        'ytp-ce-video'
                    ],
                    X: {
                        tabindex: '0',
                        'aria-label': b.NC || '',
                        'aria-hidden': 'true'
                    },
                    V: [
                        {
                            I: 'div',
                            S: 'ytp-ce-element-shadow'
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-covering-image',
                            X: S3(b)
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-covering-shadow-top'
                        },
                        {
                            I: 'a',
                            S: 'ytp-ce-covering-overlay',
                            X: {
                                href: T3(a, b.targetUrl),
                                tabindex: '-1'
                            },
                            V: [
                                {
                                    I: 'div',
                                    Ka: [
                                        'ytp-ce-video-title',
                                        'ytp-webkit-ellipsis'
                                    ],
                                    X: { dir: g.Dv(b.title || '') },
                                    xa: b.title
                                },
                                {
                                    I: 'div',
                                    S: b.pE ? 'ytp-ce-live-video-duration' : 'ytp-ce-video-duration',
                                    xa: b.videoDuration || void 0
                                }
                            ]
                        }
                    ]
                };
                c = new g.W(a);
                break;
            case 'PLAYLIST':
                a = {
                    I: 'div',
                    Ka: [
                        'ytp-ce-element',
                        'ytp-ce-playlist'
                    ],
                    X: {
                        tabindex: '0',
                        'aria-label': b.NC || '',
                        'aria-hidden': 'true'
                    },
                    V: [
                        {
                            I: 'div',
                            S: 'ytp-ce-element-shadow'
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-covering-image',
                            X: S3(b)
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-covering-shadow-top'
                        },
                        {
                            I: 'a',
                            S: 'ytp-ce-covering-overlay',
                            X: {
                                href: T3(a, b.targetUrl),
                                tabindex: '-1'
                            },
                            V: [
                                {
                                    I: 'div',
                                    Ka: [
                                        'ytp-ce-playlist-title',
                                        'ytp-webkit-ellipsis'
                                    ],
                                    X: { dir: g.Dv(b.title || '') },
                                    xa: b.title
                                },
                                {
                                    I: 'div',
                                    S: 'ytp-ce-playlist-count',
                                    V: [
                                        {
                                            I: 'div',
                                            S: 'ytp-ce-playlist-icon'
                                        },
                                        {
                                            I: 'div',
                                            S: 'ytp-ce-playlist-count-text',
                                            xa: b.playlistLength || void 0
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                };
                c = new g.W(a);
                break;
            case 'CHANNEL':
                c = {
                    I: 'div',
                    Ka: [
                        'ytp-ce-element',
                        'ytp-ce-channel',
                        b.isSubscribe ? 'ytp-ce-channel-this' : 'ytp-ce-channel-that'
                    ],
                    X: {
                        tabindex: '0',
                        'aria-label': b.NC || '',
                        'aria-hidden': 'true'
                    },
                    V: [
                        {
                            I: 'div',
                            S: 'ytp-ce-element-shadow'
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-expanding-overlay',
                            V: [
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-hider'
                                },
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-background'
                                },
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-content',
                                    V: [{
                                            I: 'div',
                                            S: 'ytp-ce-expanding-overlay-body',
                                            V: [{
                                                    I: 'div',
                                                    S: 'ytp-ce-expanding-overlay-body-padding',
                                                    V: [
                                                        {
                                                            I: 'a',
                                                            Ka: [
                                                                'ytp-ce-channel-title',
                                                                'ytp-ce-link'
                                                            ],
                                                            X: {
                                                                href: T3(a, b.targetUrl),
                                                                target: '_blank',
                                                                tabindex: '-1',
                                                                dir: g.Dv(b.title || '')
                                                            },
                                                            xa: b.title
                                                        },
                                                        b.subscribeButton ? {
                                                            I: 'div',
                                                            S: 'ytp-ce-subscribe-container',
                                                            V: [{
                                                                    I: 'div',
                                                                    S: 'ytp-ce-channel-subscribe'
                                                                }]
                                                        } : '',
                                                        b.subscribersText ? {
                                                            I: 'div',
                                                            S: 'ytp-ce-channel-subscribers-text',
                                                            xa: b.subscribersText
                                                        } : '',
                                                        b.metadata ? {
                                                            I: 'div',
                                                            Ka: [
                                                                'ytp-ce-channel-metadata',
                                                                'yt-ui-ellipsis',
                                                                'yt-ui-ellipsis-3'
                                                            ],
                                                            xa: b.metadata
                                                        } : ''
                                                    ]
                                                }]
                                        }]
                                }
                            ]
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-expanding-image',
                            X: S3(b)
                        }
                    ]
                };
                c = new g.W(c);
                var d = g.mf(document, 'div', 'ytp-ce-channel-subscribe', c.element)[0];
                if (b.subscribeButton && b.channelId) {
                    g.uv(d, 'ytp-ce-subscribe-button');
                    if (a.player.U().B) {
                        var e = null;
                        var f = b.sessionData.itct;
                    } else
                        e = 'endscreen', f = null;
                    e = new g.ZV(b.subscribeButton.subscribeText, b.subscribeButton.subscribeCount, b.subscribeButton.unsubscribeText, b.subscribeButton.unsubscribeCount, !!b.subscribeButton.enabled, !!b.subscribeButton.classic, b.channelId, !!b.subscribeButton.subscribed, e, f, a.player, !1);
                    d.appendChild(e.element);
                    a.N.push(e);
                }
                break;
            case 'WEBSITE':
                a = {
                    I: 'div',
                    Ka: [
                        'ytp-ce-element',
                        'ytp-ce-website'
                    ],
                    X: {
                        tabindex: '0',
                        'aria-label': b.NC || '',
                        'aria-hidden': 'true'
                    },
                    V: [
                        {
                            I: 'div',
                            S: 'ytp-ce-element-shadow'
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-expanding-overlay',
                            V: [
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-hider'
                                },
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-background'
                                },
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-content',
                                    V: [{
                                            I: 'div',
                                            S: 'ytp-ce-expanding-overlay-body',
                                            V: [{
                                                    I: 'div',
                                                    S: 'ytp-ce-expanding-overlay-body-padding',
                                                    V: [
                                                        {
                                                            I: 'div',
                                                            S: 'ytp-ce-website-title',
                                                            X: { dir: g.Dv(b.title || '') },
                                                            xa: b.title
                                                        },
                                                        {
                                                            I: 'div',
                                                            S: 'ytp-ce-website-metadata',
                                                            xa: b.metadata
                                                        },
                                                        {
                                                            I: 'a',
                                                            Ka: [
                                                                'ytp-ce-website-goto',
                                                                'ytp-ce-link'
                                                            ],
                                                            X: {
                                                                href: T3(a, b.targetUrl),
                                                                target: '_blank',
                                                                tabindex: '-1'
                                                            },
                                                            xa: b.callToAction
                                                        }
                                                    ]
                                                }]
                                        }]
                                }
                            ]
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-expanding-image',
                            X: S3(b)
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-expanding-icon',
                            X: Unb(b.iconUrl)
                        }
                    ]
                };
                c = new g.W(a);
                break;
            case 'CREATOR_MERCHANDISE':
                c = '', b.productPrice && (c = {
                    I: 'div',
                    S: 'ytp-ce-merchandise-price-container',
                    V: [{
                            I: 'div',
                            S: 'ytp-ce-merchandise-price',
                            xa: b.productPrice
                        }]
                }, b.additionalFeesText && c.V.push({
                    I: 'div',
                    S: 'ytp-ce-merchandise-additional-fees',
                    xa: b.additionalFeesText
                })), a = {
                    I: 'div',
                    Ka: [
                        'ytp-ce-element',
                        'ytp-ce-merchandise'
                    ],
                    X: {
                        tabindex: '0',
                        'aria-label': b.NC || '',
                        'aria-hidden': 'true'
                    },
                    V: [
                        {
                            I: 'div',
                            S: 'ytp-ce-element-shadow'
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-expanding-overlay',
                            V: [
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-hider'
                                },
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-background'
                                },
                                {
                                    I: 'div',
                                    S: 'ytp-ce-expanding-overlay-content',
                                    V: [{
                                            I: 'div',
                                            S: 'ytp-ce-expanding-overlay-body',
                                            V: [{
                                                    I: 'div',
                                                    S: 'ytp-ce-expanding-overlay-body-padding',
                                                    V: [
                                                        {
                                                            I: 'div',
                                                            S: 'ytp-ce-merchandise-title',
                                                            X: { dir: g.Dv(b.title || '') },
                                                            xa: b.title
                                                        },
                                                        c,
                                                        {
                                                            I: 'div',
                                                            S: 'ytp-ce-merchandise-metadata',
                                                            xa: b.metadata
                                                        },
                                                        {
                                                            I: 'a',
                                                            Ka: [
                                                                'ytp-ce-merchandise-goto',
                                                                'ytp-ce-link'
                                                            ],
                                                            X: {
                                                                href: T3(a, b.targetUrl),
                                                                target: '_blank',
                                                                tabindex: '-1'
                                                            },
                                                            xa: b.callToAction
                                                        }
                                                    ]
                                                }]
                                        }]
                                }
                            ]
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-expanding-image',
                            X: S3(b)
                        },
                        {
                            I: 'div',
                            S: 'ytp-ce-merchandise-invideo-cta-container',
                            V: [{
                                    I: 'div',
                                    S: 'ytp-ce-merchandise-invideo-cta',
                                    xa: b.callToAction || void 0
                                }]
                        }
                    ]
                }, c = new g.W(a);
            }
            b.isPlaceholder && g.uv(c.element, 'ytp-ce-placeholder');
            return c;
        }, S3 = function (a) {
            if (a.xW)
                var b = a.xW.thumbnails;
            return Unb(b ? b[b.length - 1].url : null);
        }, Unb = function (a) {
            return a ? { style: 'background-image: url(' + a + ')' } : {};
        }, Tnb = function (a, b, c) {
            function d(m) {
                m && (b.listen('blur', function () {
                    'none' != m.style.display && a.Y && m.focus();
                }), b.T(m, 'focus', f), b.T(m, 'blur', h));
            }
            function e(m) {
                a.K += m;
                0 < a.K ? (g.uv(b.element, 'ytp-ce-force-expand'), U3(a, c.id, !0)) : (g.wv(b.element, 'ytp-ce-force-expand'), g.wv(b.element, 'ytp-ce-element-hover'), U3(a, c.id, !1));
            }
            function f() {
                e(1);
            }
            function h() {
                e(-1);
            }
            b.listen('mouseenter', function () {
                Vnb(a, b, c);
            });
            b.listen('mouseleave', function () {
                Wnb(a, b, c);
            });
            a.player.U().B || b.listen('click', function () {
                g.uv(b.element, 'ytp-ce-element-hover');
            });
            b.listen('click', function (m) {
                Xnb(a, c, m);
            });
            b.listen('keypress', function (m) {
                Xnb(a, c, m);
            });
            b.listen('focus', function () {
                Vnb(a, b, c);
            });
            b.listen('blur', function () {
                Wnb(a, b, c);
            });
            b.listen('touchstart', function () {
                Vnb(a, b, c);
            });
            var l = g.tf('ytp-ce-expanding-overlay-hider', b.element);
            l && b.T(l, 'touchstart', function (m) {
                m.stopPropagation();
                g.wv(b.element, 'ytp-ce-element-hover');
                g.wv(b.element, 'ytp-ce-force-expand');
            });
            b.listen('keydown', function (m) {
                a.Y = 9 === m.keyCode && !m.shiftKey;
            });
            d(g.tf('ytp-sb-subscribe', b.element));
            d(g.tf('ytp-sb-unsubscribe', b.element));
            b.listen('focus', f);
            b.listen('blur', h);
        }, Xnb = function (a, b, c) {
            if (b.targetUrl && (!c || 'keypress' !== c.type || 13 === c.keyCode)) {
                for (var d = c.target; d && !g.tv(d, 'ytp-ce-element');) {
                    g.tv(d, 'subscribe-label') && Ynb(a, b);
                    if (g.tv(d, 'ytp-ce-channel-subscribe'))
                        return;
                    d = g.Gf(d);
                }
                if (!d || g.tv(d, 'ytp-ce-element-hover')) {
                    c.preventDefault();
                    c.stopPropagation();
                    if (d = a.j[b.id])
                        Wnb(a, d, b), d.element.blur();
                    if (c.ctrlKey || c.metaKey || 'new' === b.kda)
                        Ynb(a, b), a.player.sendVideoStatsEngageEvent(17, void 0), a.player.pauseVideo(), c = g.ie(T3(a, b.targetUrl)), c = g.be(c), g.NG(c, void 0, b.sessionData);
                    else {
                        var e = g.ZR(a.player.U()) || a.player.getVideoData().Kf, f = function () {
                                var h = T3(a, b.targetUrl), l = b.sessionData, m = b.watchEndpoint, n = g.lB(h);
                                e && n && (n.v || n.list) ? a.player.Io(n.v, l, n.list, !1, void 0, m || void 0) : g.Zxa(h, l);
                            };
                        Ynb(a, b, function () {
                            a.player.sendVideoStatsEngageEvent(17, f);
                        });
                    }
                }
            }
        }, T3 = function (a, b) {
            a = a.player.U();
            if (b) {
                if (b.startsWith('//'))
                    return a.protocol + ':' + b;
                if (b.startsWith('/'))
                    return g.fS(a) + b;
            } else
                return '';
            return b;
        }, Vnb = function (a, b, c) {
            g.tv(b.element, 'ytp-ce-element-hover') || ('VIDEO' === c.type || 'PLAYLIST' === c.type ? g.uv(b.element, 'ytp-ce-element-hover') : a.player.U().B ? new g.lv(function () {
                g.uv(b.element, 'ytp-ce-element-hover');
            }, 200).start() : g.uv(b.element, 'ytp-ce-element-hover'), V3(a, c.E8), U3(a, c.id, !0));
        }, Wnb = function (a, b, c) {
            g.wv(b.element, 'ytp-ce-element-hover');
            g.wv(b.element, 'ytp-ce-force-expand');
            U3(a, c.id, !1);
        }, U3 = function (a, b, c) {
            a.B && (c ? a.D.show() : a.D.hide());
            for (var d = g.u(Object.keys(a.j)), e = d.next(); !e.done; e = d.next())
                e = e.value, e !== b && g.yv(a.j[e].element, 'ytp-ce-element-shadow-show', c);
        }, V3 = function (a, b, c) {
            function d() {
                f || (e++, e === b.length && (h.stop(), c && c()));
            }
            if (!b || 0 === b.length || Q3(a))
                c && c();
            else {
                b = Znb(a, b);
                var e = 0, f = !1, h = new g.lv(function () {
                        f = !0;
                        c && c();
                    }, 1000, a);
                h.start();
                for (a = 0; a < b.length; a++)
                    g.UD(b[a], d);
            }
        }, Ynb = function (a, b, c) {
            V3(a, b.clickUrls, c);
            (a = g.aF()) && b.S8 && g.tF(a, b.visualElement);
        }, Znb = function (a, b) {
            var c = a.player.getVideoData().clientPlaybackNonce;
            a = a.player.getCurrentTime().toFixed(2);
            c = {
                CPN: c,
                AD_CPN: c,
                MT: a
            };
            a = [];
            for (var d = 0; d < b.length; d++)
                a.push($nb(b[d], c));
            return a;
        }, $nb = function (a, b) {
            return a.replace(/%5B[a-zA-Z_:]+%5D|\[[a-zA-Z_:]+\]/g, function (c) {
                var d = unescape(c);
                d = d.substring(1, d.length - 1);
                return b[d] ? escape(b[d]) : c;
            });
        }, aob = function (a) {
            return 'string' === typeof a ? a : '';
        }, W3 = function (a, b, c) {
            for (var d in b)
                if (b[d] === a)
                    return a;
            return c;
        }, bob = function (a, b, c, d) {
            this.value = a;
            this.target = b;
            this.showLinkIcon = c;
            this.j = d;
        }, X3 = function (a) {
            return a.value ? a.value : null;
        }, Y3 = function (a) {
            if (!a)
                return null;
            var b = g.ie(aob(a.value));
            b = g.be(b);
            if (!b)
                return null;
            var c = W3(a.target, cob, 'current');
            if (null == c)
                a = null;
            else {
                var d = a.show_link_icon;
                a = new bob(b, c, 'true' === d || 'false' === d ? 'true' === d : !0, null != a.pause_on_navigation ? a.pause_on_navigation : !0);
            }
            return a;
        }, dob = function (a, b, c) {
            this.type = a;
            this.trigger = b;
            this.url = c;
        }, gob = function (a) {
            if (!a)
                return null;
            var b = W3(a.type, eob), c = W3(a.trigger, fob);
            a = a.url;
            a = Array.isArray(a) && a.length ? a[0] : a;
            a = Y3(a ? a : null);
            return b ? new dob(b, c, a) : null;
        }, hob = function (a, b, c, d, e) {
            this.id = a;
            this.type = b;
            this.style = c;
            this.data = e;
            this.action = d || [];
        }, iob = function (a, b) {
            return g.Db(a.action, b);
        }, job = function (a, b) {
            this.context = a;
            this.j = b;
        }, kob = function (a) {
            return a.customMessage ? Z3('div', 'iv-card-message', a.customMessage) : '';
        }, $3 = function (a, b) {
            a = 'background-image: url(' + a + ');';
            var c = [];
            b && c.push(b);
            return {
                I: 'div',
                S: 'iv-card-image',
                X: { style: a },
                V: c
            };
        }, lob = function (a) {
            if (!a.metaInfo || 0 === a.metaInfo.length)
                return '';
            var b = [];
            a = g.u(a.metaInfo);
            for (var c = a.next(); !c.done; c = a.next())
                b.push(Z3('li', '', c.value));
            return {
                I: 'ul',
                S: 'iv-card-meta-info',
                V: b
            };
        }, Z3 = function (a, b, c) {
            b ? 'string' === typeof b ? b = { 'class': b } : Array.isArray(b) && (b = { 'class': b.join(' ') }) : b = {};
            b.dir = g.Dv(c);
            return {
                I: a,
                X: b,
                xa: c
            };
        }, mob = function (a) {
            if (!a.customMessage)
                return '';
            var b = [
                    'iv-card-action',
                    'iv-card-primary-link'
                ], c = {};
            a.YC && (b.push('iv-card-action-icon'), c.style = 'background-image: url(' + a.YC + ');');
            c.dir = g.Dv(a.customMessage);
            var d = [{
                    I: 'span',
                    xa: a.customMessage
                }];
            a.showLinkIcon && (d.push('\xA0'), d.push({
                I: 'span',
                S: 'iv-card-link-icon'
            }));
            return {
                I: 'div',
                Ka: b,
                X: c,
                V: d
            };
        }, a4 = function (a, b, c, d) {
            if (d) {
                b = g.u(b);
                for (var e = b.next(); !e.done; e = b.next())
                    a.j(e.value, d, c.id, c.sessionData, c.aj.click, 5);
            }
        }, nob = function (a, b) {
            this.merchant = a;
            this.price = b;
        }, oob = function (a) {
            var b;
            (b = a) && !(b = 1 < a.length ? '/' === a.charAt(0) && '/' !== a.charAt(1) : '/' === a) && (b = a.replace(/^(https?:)?\/\//, '').split('/', 1), b = !b || 1 > b.length || !b[0] ? [] : b[0].toLowerCase().split('.').reverse(), b = 'com' === b[0] && 'youtube' === b[1] || 'be' === b[0] && 'youtu' === b[1]);
            return b ? -1 === a.indexOf('/redirect?') : !1;
        }, pob = function (a, b) {
            return b ? b : oob(a) ? 'current' : 'new';
        }, b4 = function (a, b) {
            g.I.call(this);
            var c = this;
            this.element = a;
            this.context = b;
            this.rb = !1;
            this.Wa = new Map();
            this.Za = new Map();
            this.context.J.addEventListener(g.EJ('annotations_module'), function (d) {
                (d = c.Wa.get(d)) && d.apply(c);
            });
            this.context.J.addEventListener(g.FJ('annotations_module'), function (d) {
                (d = c.Za.get(d)) && d.apply(c);
            });
        }, c4 = function (a, b, c, d, e, f, h) {
            a.context.j.listen(b, 'click', function (l) {
                a.UL(c, d, e, f || [], h || 0, l);
            });
            a.context.j.listen(b, 'touchstart', function () {
                a.rb = !1;
            });
            a.context.j.listen(b, 'touchmove', function () {
                a.rb = !0;
            });
        }, qob = function (a) {
            var b;
            return (null == (b = g.V(a, g.TF)) ? 0 : b.url) ? g.V(a, g.TF).url : (a = g.V(a, g.WS)) && a.videoId ? (b = '/watch?v=' + a.videoId, a.playlistId && (b += '&list=' + a.playlistId), a.index && (b += '&index=' + a.index), a.startTimeSeconds && (b += '&t=' + a.startTimeSeconds), b) : null;
        }, d4 = function (a, b, c) {
            return {
                cY: (a.impressionLoggingUrlsV2s || []).map(function (d) {
                    return d.baseUrl || '';
                }),
                click: (c.loggingUrls || []).map(function (d) {
                    return d.baseUrl || '';
                }),
                close: (b.dismissLoggingUrlsV2s || []).map(function (d) {
                    return d.baseUrl || '';
                }),
                K0: (b.impressionLoggingUrlsV2s || []).map(function (d) {
                    return d.baseUrl || '';
                }),
                lL: (b.clickLoggingUrlsV2s || []).map(function (d) {
                    return d.baseUrl || '';
                })
            };
        }, rob = function (a, b, c) {
            b4.call(this, b, c);
            var d = this;
            this.J = a;
            this.eventId = null;
            this.Gb = this.Na = this.Jb = this.C = this.isInitialized = !1;
            this.cards = [];
            this.zb = this.Z = this.La = this.G = this.Va = this.j = null;
            this.qa = [];
            this.ra = this.N = this.Hf = this.Da = null;
            this.K = 0;
            this.Ga = new g.lv(function () {
            }, c.W.Fl ? 4000 : 3000);
            g.O(this, this.Ga);
            this.fb = new g.lv(function () {
            });
            g.O(this, this.fb);
            this.Ba = new job(c, function (e, f, h, l, m, n) {
                c4(d, e, f, h, l, m, n);
            });
            this.Y = new g.W({
                I: 'div',
                S: 'iv-drawer',
                X: { id: 'iv-drawer' },
                V: [
                    {
                        I: 'div',
                        S: 'iv-drawer-header',
                        X: { 'aria-role': 'heading' },
                        V: [
                            {
                                I: 'span',
                                S: 'iv-drawer-header-text'
                            },
                            {
                                I: 'button',
                                Ka: [
                                    'iv-drawer-close-button',
                                    'ytp-button'
                                ],
                                X: {
                                    'aria-label': 'Hide cards',
                                    tabindex: '0'
                                }
                            }
                        ]
                    },
                    {
                        I: 'div',
                        S: 'iv-drawer-content'
                    }
                ]
            });
            g.O(this, this.Y);
            this.D = this.Y.element;
            this.bb = new g.gG(this.Y, 330);
            g.O(this, this.bb);
            this.Hb = g.tf('iv-drawer-header-text', this.D);
            this.B = g.tf('iv-drawer-content', this.D);
            this.addCueRange(0, 1000 * c.videoData.lengthSeconds, '', function () {
                d.Jb && e4(d, 'YOUTUBE_DRAWER_AUTO_OPEN');
            }, function () {
                (d.Jb = d.C) && f4(d);
            });
            this.Ra = new g.vK(this);
            g.O(this, this.Ra);
            this.J.addEventListener('videodatachange', this.jt.bind(this));
        }, sob = function (a, b) {
            b = b.data;
            b.autoOpenMs && a.addCueRange(b.autoOpenMs, 2251799813685248, '', function () {
                e4(a, 'YOUTUBE_DRAWER_AUTO_OPEN');
            });
            b.autoCloseMs && a.addCueRange(b.autoCloseMs, 2251799813685248, '', function () {
                f4(a);
            });
            var c = b.headerText;
            g.Of(a.Hb, c);
            a.Z && a.Z.setAttribute('title', c);
            b.eventId && (a.eventId = b.eventId);
            a.Da = g.ZE(b.trackingParams);
            a.N = g.ZE(b.closeTrackingParams);
            a.Hf = g.ZE(b.iconTrackingParams);
        }, tob = function (a, b) {
            var c = b.cardId ? b.cardId : 'cr:' + a.K, d = a.J.U().experiments.jb('enable_error_corrections_infocard_web_client');
            if (!b.content && b.teaser.simpleCardTeaserRenderer && d) {
                var e = b.teaser.simpleCardTeaserRenderer, f = b.icon ? b.icon.infoCardIconRenderer : null;
                b = {
                    id: c,
                    timestamp: a.K,
                    type: 'simple',
                    teaserText: g.HF(e.message),
                    teaserDurationMs: Number(b.cueRanges[0].teaserDurationMs),
                    startMs: Number(b.cueRanges[0].startCardActiveMs),
                    autoOpen: !!b.autoOpen,
                    sessionData: {},
                    sponsored: !1,
                    aj: {},
                    Zo: null,
                    Gj: e.trackingParams ? g.ZE(e.trackingParams) : null,
                    Hf: f && f.trackingParams ? g.ZE(f.trackingParams) : null,
                    imageUrl: '',
                    displayDomain: null,
                    showLinkIcon: !1,
                    YC: null,
                    title: '',
                    customMessage: '',
                    url: null,
                    onClickCommand: e.onTapCommand || null
                };
                g4(a, b);
            } else {
                var h;
                if (null == (h = b.content) ? 0 : h.simpleCardContentRenderer) {
                    if (!b.cueRanges.length)
                        return;
                    f = null == (e = b.content) ? void 0 : e.simpleCardContentRenderer;
                    e = b.teaser.simpleCardTeaserRenderer;
                    var l = b.icon ? b.icon.infoCardIconRenderer : null;
                    b = {
                        id: c,
                        timestamp: a.K,
                        type: 'simple',
                        teaserText: g.HF(e.message),
                        teaserDurationMs: Number(b.cueRanges[0].teaserDurationMs),
                        startMs: Number(b.cueRanges[0].startCardActiveMs),
                        autoOpen: !!b.autoOpen,
                        sessionData: h4(a, c, b, f),
                        sponsored: !1,
                        aj: d4(f, e, f.command),
                        Zo: f.trackingParams ? g.ZE(f.trackingParams) : null,
                        Gj: e.trackingParams ? g.ZE(e.trackingParams) : null,
                        Hf: l && l.trackingParams ? g.ZE(l.trackingParams) : null,
                        imageUrl: i4(f.image.thumbnails, 290).url,
                        displayDomain: f.displayDomain ? g.HF(f.displayDomain) : null,
                        showLinkIcon: !!f.showLinkIcon,
                        YC: null,
                        title: f.title ? g.HF(f.title) : '',
                        customMessage: f.callToAction ? g.HF(f.callToAction) : '',
                        url: g.V(f.command, g.TF).url ? Y3({
                            pause_on_navigation: !a.context.videoData.isLivePlayback,
                            target: 'new',
                            value: g.V(f.command, g.TF).url
                        }) : null,
                        onClickCommand: null
                    };
                    g4(a, b);
                } else {
                    var m;
                    if (null == (m = b.content) ? 0 : m.collaboratorInfoCardContentRenderer) {
                        if (!b.cueRanges.length)
                            return;
                        e = null == (f = b.content) ? void 0 : f.collaboratorInfoCardContentRenderer;
                        f = b.teaser.simpleCardTeaserRenderer;
                        l = b.icon ? b.icon.infoCardIconRenderer : null;
                        b = {
                            id: c,
                            timestamp: a.K,
                            type: 'collaborator',
                            teaserText: g.HF(f.message),
                            teaserDurationMs: Number(b.cueRanges[0].teaserDurationMs),
                            startMs: Number(b.cueRanges[0].startCardActiveMs),
                            autoOpen: !!b.autoOpen,
                            sessionData: h4(a, c, b, e),
                            sponsored: !1,
                            aj: d4(e, f, e.endpoint),
                            Zo: e.trackingParams ? g.ZE(e.trackingParams) : null,
                            Gj: f.trackingParams ? g.ZE(f.trackingParams) : null,
                            Hf: l && l.trackingParams ? g.ZE(l.trackingParams) : null,
                            channelId: g.V(e.endpoint, g.kT).browseId,
                            customMessage: e.customText ? g.HF(e.customText) : null,
                            profileImageUrl: i4(e.channelAvatar.thumbnails, 290).url,
                            title: e.channelName ? g.HF(e.channelName) : '',
                            metaInfo: [e.subscriberCountText ? g.HF(e.subscriberCountText) : ''],
                            url: Y3({
                                pause_on_navigation: !a.context.videoData.isLivePlayback,
                                target: 'new',
                                value: g.V(e.endpoint, g.kT).canonicalBaseUrl ? g.V(e.endpoint, g.kT).canonicalBaseUrl : '/channel/' + g.V(e.endpoint, g.kT).browseId
                            }),
                            onClickCommand: null
                        };
                        g4(a, b);
                    } else {
                        var n;
                        if (null == (n = b.content) ? 0 : n.playlistInfoCardContentRenderer) {
                            if (!b.cueRanges.length)
                                return;
                            e = null == (l = b.content) ? void 0 : l.playlistInfoCardContentRenderer;
                            f = b.teaser.simpleCardTeaserRenderer;
                            l = b.icon ? b.icon.infoCardIconRenderer : null;
                            b = {
                                id: c,
                                timestamp: a.K,
                                type: 'playlist',
                                teaserText: g.HF(f.message),
                                teaserDurationMs: Number(b.cueRanges[0].teaserDurationMs),
                                startMs: Number(b.cueRanges[0].startCardActiveMs),
                                autoOpen: !!b.autoOpen,
                                sessionData: h4(a, c, b, e),
                                sponsored: !1,
                                aj: d4(e, f, e.action),
                                Zo: e.trackingParams ? g.ZE(e.trackingParams) : null,
                                Gj: f.trackingParams ? g.ZE(f.trackingParams) : null,
                                Hf: l && l.trackingParams ? g.ZE(l.trackingParams) : null,
                                CG: i4(e.playlistThumbnail.thumbnails, 258).url,
                                customMessage: e.customMessage ? g.HF(e.customMessage) : null,
                                playlistVideoCount: g.HF(e.playlistVideoCount),
                                title: e.playlistTitle ? g.HF(e.playlistTitle) : '',
                                metaInfo: [
                                    e.channelName ? g.HF(e.channelName) : '',
                                    e.videoCountText ? g.HF(e.videoCountText) : ''
                                ],
                                url: Y3({
                                    pause_on_navigation: !a.context.videoData.isLivePlayback,
                                    target: 'new',
                                    value: qob(e.action)
                                }),
                                onClickCommand: null
                            };
                            g4(a, b);
                        } else {
                            var p;
                            if (null == (p = b.content) ? 0 : p.videoInfoCardContentRenderer) {
                                if (!b.cueRanges.length)
                                    return;
                                var q;
                                e = null == (q = b.content) ? void 0 : q.videoInfoCardContentRenderer;
                                f = b.teaser.simpleCardTeaserRenderer;
                                l = b.icon ? b.icon.infoCardIconRenderer : null;
                                b = {
                                    id: c,
                                    timestamp: a.K,
                                    type: 'video',
                                    teaserText: g.HF(f.message),
                                    teaserDurationMs: Number(b.cueRanges[0].teaserDurationMs),
                                    startMs: Number(b.cueRanges[0].startCardActiveMs),
                                    autoOpen: !!b.autoOpen,
                                    sessionData: h4(a, c, b, e),
                                    sponsored: !1,
                                    aj: d4(e, f, e.action),
                                    Zo: e.trackingParams ? g.ZE(e.trackingParams) : null,
                                    Gj: f.trackingParams ? g.ZE(f.trackingParams) : null,
                                    Hf: l && l.trackingParams ? g.ZE(l.trackingParams) : null,
                                    CG: i4(e.videoThumbnail.thumbnails, 258).url,
                                    videoDuration: e.lengthString ? g.HF(e.lengthString) : null,
                                    customMessage: e.customMessage ? g.HF(e.customMessage) : null,
                                    title: e.videoTitle ? g.HF(e.videoTitle) : '',
                                    metaInfo: [
                                        e.channelName ? g.HF(e.channelName) : '',
                                        e.viewCountText ? g.HF(e.viewCountText) : ''
                                    ],
                                    isLiveNow: !!e.badge,
                                    url: Y3({
                                        pause_on_navigation: !a.context.videoData.isLivePlayback,
                                        target: 'new',
                                        value: qob(e.action)
                                    }),
                                    onClickCommand: null
                                };
                                g4(a, b);
                            }
                        }
                    }
                }
            }
            a.K++;
        }, i4 = function (a, b) {
            for (var c = -1, d = -1, e = 0; e < a.length; e++) {
                if (a[e].height === b || 290 === a[e].width)
                    return a[e];
                ((a[e].height || 0) < b || 290 > (a[e].width || 0)) && (0 > c || (a[c].height || 0) < (a[e].height || 0) || (a[c].width || 0) < (a[e].width || 0)) ? c = e : ((a[e].height || 0) >= b || 290 <= (a[e].width || 0)) && (0 > d || (a[d].height || 0) > (a[e].height || 0) || (a[d].width || 0) > (a[e].width || 0)) && (d = e);
            }
            return a[0 <= d ? d : c];
        }, h4 = function (a, b, c, d) {
            return {
                feature: c.feature ? c.feature : 'cards',
                src_vid: a.context.videoData.videoId,
                annotation_id: b,
                ei: a.context.videoData.eventId || '',
                itct: d.trackingParams || ''
            };
        }, vob = function (a, b) {
            if (b = uob(a, b))
                b === a.j && (a.j = null), a.J.removeCueRange(b.Cf.id), g.Ff(b.qN), g.Hb(a.cards, b), a.zH(), j4(a);
        }, e4 = function (a, b, c) {
            if (!a.C) {
                a.bb.show();
                a.Va = new g.lv(function () {
                    g.uv(a.context.J.getRootNode(), g.MY.IV_DRAWER_OPEN);
                }, 0);
                a.Va.start();
                a.Ra.T(a.B, 'mousewheel', function (h) {
                    a.Ga.start();
                    h.preventDefault();
                    h = h || window.event;
                    var l = 0;
                    'MozMousePixelScroll' == h.type ? l = 0 == (h.axis == h.HORIZONTAL_AXIS) ? h.detail : 0 : window.opera ? l = h.detail : l = 0 == h.wheelDelta % 120 ? 'WebkitTransform' in document.documentElement.style ? window.chrome && 0 == navigator.platform.indexOf('Mac') ? h.wheelDeltaY / -30 : h.wheelDeltaY / -1.2 : h.wheelDelta / -1.6 : h.wheelDeltaY / -3;
                    if (h = l)
                        a.B.scrollTop += h;
                });
                a.C = !0;
                var d = g.aF();
                d && a.Da && a.N && g.rF(d, [
                    a.Da,
                    a.N
                ]);
                b = { TRIGGER_TYPE: b };
                for (var e = g.u(a.cards), f = e.next(); !f.done; f = e.next())
                    f = f.value, f.WY || (f.WY = !0, wob(a.context.logger, f.Cf.aj.cY, b)), d && g.rF(d, [f.Cf.Zo]);
                H3(a.J);
                c && (a.G = new g.lv(function () {
                    a.La = a.Z;
                    a.zb.focus();
                }, 330), a.G.start());
            }
        }, f4 = function (a) {
            a.C && (a.bb.hide(), g.pC(a.Ra), g.wv(a.context.J.getRootNode(), g.MY.IV_DRAWER_OPEN), a.C = !1, H3(a.J), a.G && a.G.stop(), a.G = new g.lv(function () {
                a.La && (a.La.focus(), a.La = null);
            }, 330), a.G.start());
        }, yob = function (a) {
            g.vv(a.Ib(), [
                g.MY.STOP_EVENT_PROPAGATION,
                'iv-drawer-manager'
            ]);
            g.cU(a.J, a.D, 5);
            xob(a);
            a.Z = g.tf('ytp-cards-button', a.J.getRootNode());
            a.zb = g.tf('iv-drawer-close-button', a.D);
            a.isInitialized = !0;
        }, xob = function (a) {
            var b = g.tf('iv-drawer-close-button', a.D);
            a.context.j.listen(b, 'click', a.q6, a);
            a.context.j.listen(a.B, 'touchend', function () {
                a.Ga.start();
            });
            a.context.j.listen(a.B, 'scroll', a.N6, a);
            a.context.B.subscribe('onHideControls', function () {
                a.Na = !0;
            });
            a.context.B.subscribe('onShowControls', function () {
                a.Na = !1;
            });
            a.context.B.subscribe('onVideoAreaChange', function () {
                a.Na = g.tv(a.J.getRootNode(), 'ytp-autohide');
            });
            a.qa.push(g.hE('iv-button-shown', a.k9, a));
            a.qa.push(g.hE('iv-button-hidden', a.j9, a));
            zob(a);
        }, zob = function (a) {
            a.qa.push(g.hE('iv-teaser-shown', a.n3, a));
            a.qa.push(g.hE('iv-teaser-hidden', a.l9, a));
            a.qa.push(g.hE('iv-teaser-clicked', a.m3, a));
            a.Gb = !0;
        }, Aob = function (a, b) {
            var c;
            return b.onClickCommand && 'engagement-panel-error-corrections' === (null == (c = g.V(b.onClickCommand, g.bkb)) ? void 0 : c.targetId) ? (a.ra = b, !0) : !1;
        }, Bob = function (a, b) {
            a.ra = b;
            var c = a.J.getVideoData();
            if (!c)
                return !1;
            c = g.bTa(c);
            if (null == c ? 0 : c.markersMap)
                for (var d, e = 0; (null == (d = c) ? void 0 : d.markersMap.length) > e; e++) {
                    var f = void 0, h = null == (f = c) ? void 0 : f.markersMap[e];
                    if ('ERROR_CORRECTION_MARKERS' === h.key && (f = void 0, (h = null == (f = h.value) ? void 0 : f.markers) && 0 < h.length))
                        return d = void 0, b.startMs = (null == (d = g.V(h[0], g.B0a)) ? void 0 : d.timeRangeStartMillis) || 0, a.ra = null, !0;
                }
            return !1;
        }, g4 = function (a, b) {
            if (!Aob(a, b) || Bob(a, b)) {
                var c = Cob(a, b);
                if (c) {
                    var d = {
                        Cf: b,
                        qN: c.element,
                        WY: !1
                    };
                    if (b.onClickCommand)
                        a.J.L('web_infocards_teaser_show_logging_fix') && (a.Gb || zob(a), vob(a, b.id), c = a.findLastIndex(d), g.Pb(a.cards, c, 0, d));
                    else {
                        a.isInitialized || yob(a);
                        vob(a, b.id);
                        var e = a.findLastIndex(d);
                        g.Pb(a.cards, e, 0, d);
                        c.Ja(a.B, e);
                        a.zH();
                    }
                    b.autoOpen ? a.addCueRange(b.startMs, 2251799813685248, b.id, function () {
                        a.C || (a.j = d, j4(a), Dob(a, d), e4(a, 'YOUTUBE_DRAWER_AUTO_OPEN', !1));
                    }) : (c = 1000 * a.context.J.getCurrentTime(), 5000 > c && c > b.startMs && Eob(a, d), a.addCueRange(b.startMs, b.startMs + 1, b.id, function () {
                        Eob(a, d);
                    }), j4(a));
                }
            }
        }, Cob = function (a, b) {
            switch (b.type) {
            case 'simple':
                a = a.Ba;
                var c = b.displayDomain ? {
                    I: 'div',
                    S: 'iv-card-image-text',
                    xa: b.displayDomain
                } : void 0;
                var d = mob(b);
                c = {
                    I: 'div',
                    Ka: ['iv-card'],
                    V: [{
                            I: 'a',
                            S: 'iv-click-target',
                            X: { href: b.url ? X3(b.url) || '' : '' },
                            V: [
                                $3(b.imageUrl, c),
                                {
                                    I: 'div',
                                    S: 'iv-card-content',
                                    V: [
                                        Z3('h2', void 0, b.title),
                                        d
                                    ]
                                }
                            ]
                        }]
                };
                c = new g.W(c);
                a4(a, g.nf('iv-click-target', c.element), b, b.url);
                b = c;
                break;
            case 'collaborator':
                a = a.Ba;
                c = {
                    I: 'div',
                    Ka: [
                        'iv-card',
                        'iv-card-channel'
                    ],
                    V: [{
                            I: 'a',
                            Ka: ['iv-click-target'],
                            X: {
                                href: X3(b.url) || '',
                                'data-ytid': b.channelId
                            },
                            V: [
                                $3(b.profileImageUrl),
                                {
                                    I: 'div',
                                    S: 'iv-card-content',
                                    V: [
                                        kob(b),
                                        {
                                            I: 'h2',
                                            S: 'iv-card-primary-link',
                                            xa: b.title
                                        },
                                        lob(b)
                                    ]
                                }
                            ]
                        }]
                };
                c = new g.W(c);
                a4(a, g.nf('iv-click-target', c.element), b, b.url);
                b = c;
                break;
            case 'playlist':
                a = a.Ba;
                c = {
                    I: 'div',
                    Ka: [
                        'iv-card',
                        'iv-card-playlist'
                    ],
                    V: [{
                            I: 'a',
                            S: 'iv-click-target',
                            X: { href: X3(b.url) || '' },
                            V: [
                                $3(b.CG, {
                                    I: 'div',
                                    S: 'iv-card-image-overlay',
                                    V: [{
                                            I: 'span',
                                            S: 'iv-card-playlist-video-count',
                                            xa: b.playlistVideoCount
                                        }]
                                }),
                                {
                                    I: 'div',
                                    S: 'iv-card-content',
                                    V: [
                                        kob(b),
                                        Z3('h2', 'iv-card-primary-link', b.title),
                                        lob(b)
                                    ]
                                }
                            ]
                        }]
                };
                c = new g.W(c);
                a4(a, g.nf('iv-click-target', c.element), b, b.url);
                b = c;
                break;
            case 'productListing':
                a = a.Ba;
                var e = 0 != b.offers.length;
                c = ['iv-card'];
                d = '';
                var f = mob(b);
                e && (c.push('iv-card-product-listing'), d = 'iv-card-primary-link', f = b.offers[0], e = [], f.price && e.push({
                    I: 'div',
                    S: 'iv-card-offer-price',
                    xa: f.price
                }), f.merchant && e.push({
                    I: 'div',
                    S: 'iv-card-offer-merchant',
                    xa: f.merchant
                }), f = {
                    I: 'div',
                    V: e
                });
                e = b.url ? X3(b.url) || '' : '';
                c = {
                    I: 'div',
                    Ka: c,
                    X: { tabindex: '0' },
                    V: [
                        {
                            I: 'a',
                            Ka: [
                                'iv-card-image',
                                'iv-click-target'
                            ],
                            X: {
                                style: 'background-image: url(' + b.imageUrl + ');',
                                href: e,
                                'aria-hidden': 'true',
                                tabindex: '-1'
                            }
                        },
                        {
                            I: 'div',
                            S: 'iv-card-content',
                            V: [
                                b.sponsored ? {
                                    I: 'div',
                                    S: 'iv-card-sponsored',
                                    V: [
                                        'Sponsored',
                                        {
                                            I: 'div',
                                            S: 'iv-ad-info-container',
                                            V: [
                                                {
                                                    I: 'div',
                                                    S: 'iv-ad-info',
                                                    xa: '{{adInfo}}'
                                                },
                                                {
                                                    I: 'div',
                                                    S: 'iv-ad-info-icon-container',
                                                    V: [
                                                        {
                                                            I: 'div',
                                                            S: 'iv-ad-info-icon'
                                                        },
                                                        {
                                                            I: 'div',
                                                            S: 'iv-ad-info-callout'
                                                        }
                                                    ]
                                                }
                                            ]
                                        }
                                    ]
                                } : '',
                                {
                                    I: 'a',
                                    S: 'iv-click-target',
                                    X: { href: e },
                                    V: [
                                        Z3('h2', d, b.title),
                                        f
                                    ]
                                }
                            ]
                        }
                    ]
                };
                c = new g.W(c);
                d = g.yf('span');
                g.Of(d, 'You are seeing this product because we think it is relevant to the video. Google may be compensated by the merchant.');
                c.Ld(d, 'adInfo');
                a4(a, g.nf('iv-click-target', c.element), b, b.url);
                b = c;
                break;
            case 'video':
                a = a.Ba;
                d = b.videoDuration ? {
                    I: 'span',
                    S: 'iv-card-video-duration',
                    xa: b.videoDuration
                } : void 0;
                f = b.isLiveNow ? {
                    I: 'span',
                    Ka: [
                        'yt-badge',
                        'yt-badge-live'
                    ],
                    xa: 'LIVE NOW'
                } : null;
                e = {
                    I: 'div',
                    Ka: [
                        'iv-card',
                        'iv-card-video'
                    ],
                    V: [{
                            I: 'a',
                            S: 'iv-click-target',
                            X: { href: (null == (c = b.url) ? void 0 : X3(c)) || '' },
                            V: [
                                $3(b.CG, d),
                                {
                                    I: 'div',
                                    S: 'iv-card-content',
                                    V: [
                                        kob(b),
                                        Z3('h2', 'iv-card-primary-link', b.title),
                                        lob(b),
                                        f
                                    ]
                                }
                            ]
                        }]
                };
                c = new g.W(e);
                a4(a, g.nf('iv-click-target', c.element), b, b.url);
                b = c;
                break;
            default:
                return null;
            }
            return b;
        }, Fob = function (a) {
            if (a.j)
                return 'productListing' === a.j.Cf.type;
            if (a.J.L('enable_wn_infocards')) {
                var b;
                return !(null == (b = a.cards) || !b.length) && g.Sr(a.cards, function (c) {
                    return 'productListing' === c.Cf.type;
                });
            }
            return g.Sr(a.cards, function (c) {
                return 'productListing' === c.Cf.type;
            });
        }, j4 = function (a) {
            g.yv(a.J.getRootNode(), 'ytp-cards-shopping-active', Fob(a));
        }, Eob = function (a, b) {
            if (!g.tv(a.J.getRootNode(), 'ytp-cards-teaser-shown')) {
                if (a.j !== b) {
                    var c = g.aF(), d = a.j ? a.j.Cf.Hf : a.Hf;
                    c && d && g.sF(c, [d]);
                    a.j = b;
                    j4(a);
                }
                (c = a.isInitialized && 'none' == a.Ib().style.display) || (c = a.context.J.getPlayerState(), d = 0 === c && 0 === a.context.J.getCurrentTime(), c = !(1 === c || 3 === c || d));
                c || b.Cf.teaserDurationMs && a.J.rB(!0, {
                    teaserText: b.Cf.teaserText,
                    durationMs: b.Cf.teaserDurationMs,
                    onClickCommand: b.Cf.onClickCommand
                });
                a.fb.isActive() || ((!a.C || !a.Ga.isActive() && a.Na) && Dob(a, b), a.fb.start(910 + b.Cf.teaserDurationMs));
            }
        }, Dob = function (a, b) {
            a.Y.Eb ? (b = new M3([
                0,
                a.B.scrollTop
            ], [
                0,
                b.qN.offsetTop
            ], 600, Knb), a.context.C.listen(b, 'animate', function (c) {
                a.B.scrollTop = c.y;
            }), a.context.C.listen(b, 'finish', function (c) {
                a.B.scrollTop = c.y;
            }), b.play()) : (g.DF(a.Y, !0), a.B.scrollTop = b.qN.offsetTop, g.DF(a.Y, !1));
        }, k4 = function (a) {
            return a.j && a.j.Cf ? a.j.Cf : a.cards[0] && a.cards[0].Cf ? a.cards[0].Cf : null;
        }, uob = function (a, b) {
            return g.Db(a.cards, function (c) {
                return c.Cf.id === b;
            });
        }, l4 = function (a, b, c) {
            b4.call(this, a, b);
            this.annotation = c;
            this.isActive = !1;
        }, Gob = function (a) {
            var b = a.annotation.data;
            'start_ms' in b && 'end_ms' in b && a.addCueRange(b.start_ms, b.end_ms, a.annotation.id, function () {
                a.show();
            }, function () {
                a.hide();
            });
        }, m4 = function (a, b, c) {
            l4.call(this, a, b, c);
            this.B = null;
            this.N = !1;
            this.C = null;
            this.D = !1;
            this.j = this.K = this.G = null;
        }, Hob = function (a, b) {
            var c = void 0 === c ? 0 : c;
            var d = Bnb(b).width;
            g.Gs(b, d);
            c = new Inb(b, [
                d,
                b.offsetTop
            ], [
                d - d - c,
                b.offsetTop
            ], 200, Jnb);
            g.O(a, c);
            a.context.C.listen(c, 'begin', function () {
                g.Ms(b, !0);
            });
            c.play();
        }, Kob = function (a, b) {
            if (b.channel_name) {
                var c = a.createElement({
                        I: 'div',
                        Ka: ['iv-branding-context-name'],
                        xa: b.channel_name
                    }), d = a.createElement({
                        I: 'div',
                        Ka: ['iv-branding-context-subscribe']
                    }), e = b.standalone_subscribe_button_data;
                e && (a.j = new g.ZV(e.subscribeText, e.subscribeCount, e.unsubscribeText, e.unsubscribeCount, !!e.enabled, !!e.classic, b.channel_id, !!e.subscribed, e.feature, b.session_data.itct, a.context.J, !1), a.j.Ja(d));
                var f = a.createElement({
                        I: 'div',
                        Ka: ['iv-branding-context-subscribe-caret']
                    }), h = a.createElement({
                        I: 'div',
                        Ka: ['branding-context-container-inner']
                    });
                h.appendChild(f);
                h.appendChild(c);
                h.appendChild(d);
                g.Ms(h, !1);
                var l = a.createElement({
                    I: 'div',
                    Ka: ['branding-context-container-outer']
                });
                l.appendChild(h);
                g.zs(l, 'right', b.image_width + 'px');
                g.Af(a.Ib(), l);
                a.C = new g.lv(function () {
                    Iob(a, h, l);
                }, 500);
                g.O(a, a.C);
                a.context.j.listen(a.Ib(), 'mouseover', function () {
                    Job(a, h, l, f, b.image_height);
                });
                a.context.j.listen(a.Ib(), 'mouseout', function () {
                    a.C.start();
                });
            }
        }, Job = function (a, b, c, d, e) {
            a.C.stop();
            if (!a.D) {
                var f = g.Ls(b);
                a.j || (b.style.width = g.Fs(f.width, !0), c.style.width = g.Fs(f.width, !0));
                g.zs(d, 'top', f.height - Math.max(Math.min(f.height, e) / 2 + 10, 20) + 'px');
                g.zs(d, 'right', '1px');
                a.D = !0;
                g.Ms(b, !0);
                a.G = new g.lv(function () {
                    g.uv(this.Ib(), 'iv-branding-active');
                }, 0, a);
                a.G.start();
            }
        }, Iob = function (a, b, c) {
            g.wv(a.Ib(), 'iv-branding-active');
            a.K = new g.lv(function () {
                g.Ms(b, !1);
                a.j || (c.style.width = g.Fs(0, !0));
            }, 250);
            a.K.start();
            a.D = !1;
        }, Lob = function (a, b, c, d, e, f, h) {
            this.j = a;
            this.C = b;
            this.W = c;
            this.videoData = d;
            this.logger = e;
            this.J = f;
            this.B = h;
        }, Mob = function (a, b, c) {
            l4.call(this, a, b, c);
            var d = this;
            this.Z = this.isCollapsed = this.Y = !1;
            this.K = 5000;
            this.B = this.C = this.j = this.D = null;
            this.N = this.createElement({
                I: 'div',
                Ka: ['iv-promo-contents']
            });
            this.G = new g.lv(function () {
                d.j.setAttribute('aria-hidden', 'true');
                g.Ms(d.C, !1);
                g.Ms(d.B, !0);
            }, 700, this);
            g.O(this, this.G);
        }, Pob = function (a, b, c) {
            c.stopPropagation();
            Nob(a);
            Oob(a, b);
            a.j.focus();
        }, Qob = function (a) {
            a.isCollapsed || a.Z || a.D || (g.uv(a.Ib(), 'iv-promo-collapsed'), a.isCollapsed = !0, a.G.start());
        }, Nob = function (a) {
            a.G.stop();
            a.isCollapsed && (g.xv(a.Ib(), [
                'iv-promo-collapsed',
                'iv-promo-collapsed-no-delay'
            ]), a.isCollapsed = !1, a.j && a.j.removeAttribute('aria-hidden'), g.Ms(a.B, !1), g.Ms(a.C, !0));
        }, Oob = function (a, b) {
            a.D || (a.D = g.Bg(function () {
                Rob(this);
                Qob(this);
            }, b, a));
        }, Rob = function (a) {
            a.D && (g.Ta.clearTimeout(a.D), a.D = null);
        }, Sob = function (a) {
            this.J = a;
        }, wob = function (a, b, c) {
            b && (c ? n4(a, b.map(function (d) {
                return g.Wu(d, c);
            })) : n4(a, b));
        }, n4 = function (a, b, c, d) {
            var e = 1, f = void 0, h = -1;
            if (c) {
                var l = !1;
                f = function () {
                    e--;
                    e || l || (clearTimeout(h), l = !0, c());
                };
                h = setTimeout(function () {
                    l = !0;
                    c();
                }, 1000);
            }
            b = g.u(b || []);
            for (var m = b.next(); !m.done; m = b.next())
                m = m.value, e++, g.UD(m, f);
            d && (e++, 0 !== d && a.J.sendVideoStatsEngageEvent(d, f));
        }, Tob = function (a) {
            g.qV.call(this, a);
            var b = this;
            this.qa = this.Z = !1;
            this.loadNumber = 0;
            this.N = {};
            this.logger = new Sob(this.player);
            this.D = new g.vK(this);
            this.G = this.K = null;
            this.events = new g.vK(this);
            this.Sg = this.Y = this.j = null;
            this.ra = [];
            g.O(this, this.D);
            this.D.T(this.player, 'onVideoAreaChange', function () {
                b.publish('onVideoAreaChange');
            });
            this.D.T(this.player, 'onHideControls', function () {
                b.publish('onHideControls');
            });
            this.D.T(this.player, 'onShowControls', function () {
                b.publish('onShowControls');
            });
            this.D.T(this.player, 'resize', function (d) {
                b.publish('resize', d);
            });
            this.D.T(this.player, 'presentingplayerstatechange', function (d) {
                b.publish('presentingplayerstatechange', d);
            });
            this.subscribe('presentingplayerstatechange', this.o3, this);
            this.subscribe('resize', this.cK, this);
            this.player.U().Na.subscribe('vast_info_card_add', this.a_, this);
            g.O(this, this.events);
            this.Ba = this.createElement({
                I: 'div',
                S: 'video-custom-annotations'
            });
            this.B = new g.W({
                I: 'div',
                Ka: [
                    'ytp-player-content',
                    'ytp-iv-player-content'
                ]
            });
            g.O(this, this.B);
            g.cU(this.player, this.B.element, 4);
            this.B.hide();
            this.C = new g.W({
                I: 'div',
                Ka: ['ytp-iv-video-content']
            });
            g.O(this, this.C);
            a = this.createElement({
                I: 'div',
                S: 'video-annotations'
            });
            a.appendChild(this.Ba);
            this.C.element.appendChild(a);
            this.ju() && this.load();
            var c = this.createElement({ I: 'style' });
            (g.lf('HEAD')[0] || document.body).appendChild(c);
            this.addOnDisposeCallback(function () {
                g.Ff(c);
            });
            if (a = c.sheet)
                a.insertRule('.iv-promo .iv-promo-contents .iv-promo-txt .iv-promo-link:after {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUBAMAAAB/pwA+AAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAHlBMVEVMaXH////////////////////////////////////Z6AnKAAAACXRSTlMA+/A2IuI1mJIldm0CAAAAAWJLR0QB/wIt3gAAAEVJREFUCNdjYGCYCQUMBJlACOIzIDElIcyZkwxgojOVWWDMSQauMKYySySUOSnBdSaUOZ0lEsac2YqwYiZ+JhwgM7E5HACgzVCI/YJ59AAAAABJRU5ErkJggg==) no-repeat center;background-size:10px;width:10px;height:10px}', 0), a.insertRule('.iv-promo .iv-promo-actions .iv-promo-close:after {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAkAAAAJBAMAAAASvxsjAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAJFBMVEVMaXH///////////////////////////////////////////9tKdXLAAAAC3RSTlMAVaQDpaimqQbl5rjXUFUAAAABYktHRAH/Ai3eAAAAPUlEQVQI12MQMmAwEmDwDmaOTmAw39663YCBuXp2MQMDQ+fOBgYG5ujVwQwMptvbgeLaxczVCQwiBgxmAgBkXg1FN5iwiAAAAABJRU5ErkJggg==) no-repeat center;background-size:9px;width:9px;height:9px}', 0), a.insertRule('.iv-promo .iv-promo-actions .iv-promo-expand:after {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAQAAAAJBAMAAADnQZCTAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAJFBMVEVMaXHMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMz////eMKB4AAAAC3RSTlMAOpE7k5Uvj5kpfRaQSaQAAAABYktHRAsf18TAAAAAHklEQVQI12MQYGBQZmBwTWCo0GSo6AKRQDZQRIABADXXA/UkIpvtAAAAAElFTkSuQmCC) no-repeat center;background-size:4px 9px;width:4px;height:9px}', 0), a.insertRule('.iv-promo-website-card-cta-redesign .iv-promo-round-expand-icon:after {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAQAAAD9CzEMAAAAAmJLR0QA/4ePzL8AAAAJcEhZcwAACxMAAAsTAQCanBgAAAAHdElNRQfgCgUUEztsNfqrAAAAXklEQVRYw+3Uuw2AQAwEUUNXfBpDIvBRMhQwJJAScNrA0r4CdiQHjjAzK4NGKucPAFmCnZcmwcTphBNO9CTGH4VB+/Zm6YlYis9fhedXz38FNvFriCCl808iw8ysrBu65gCeuV/CfgAAAABJRU5ErkJggg==) no-repeat center;background-size:18px 18px;width:18px;height:18px}', 0), a.insertRule('.iv-card-link-icon {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABIAAAASBAMAAACk4JNkAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAGFBMVEVMaXG7u7u7u7u7u7u7u7u7u7u7u7v///+WKTAlAAAABnRSTlMAFdQWbGj9GiOuAAAAAWJLR0QHFmGI6wAAAEhJREFUCNdjYACBNCBgQGMxMKrBWEJJaRAJRjVlKEsoSQDIAqtSZICwgEIQFkgIZBRECMxiBqsCsVjAqsCygQwwFgMeFgQgswBg2xjLrfC4mgAAAABJRU5ErkJggg==) no-repeat center;background-size:9px;width:9px;height:9px}', 0), a.insertRule('.iv-card-playlist-video-count:after {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYBAMAAAASWSDLAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAJFBMVEVMaXH///////////////////////////////////////////9tKdXLAAAAC3RSTlMAvDeyLvxYtDK9Ogx4T1QAAAABYktHRAH/Ai3eAAAAK0lEQVQY02NgoBjshgO8HJoYwKiAMGAD92YHJM7uMCTO9gaEHs4FlPuZAQC8Fj8x/xHjxwAAAABJRU5ErkJggg==) no-repeat center;background-size:24px;width:24px;height:24px}', 0), a.insertRule('.iv-drawer-close-button:after {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAwAAAAMAgMAAAArG7R0AAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAACVBMVEVMaXH////////OZTV/AAAAAnRSTlMAoKBFbtAAAAABYktHRAH/Ai3eAAAAKUlEQVQI12MIYGBlSGGQBMIUBjbHCQyM0xwYGDIZwBjEBomB5EBqgGoBolQGzYuy51cAAAAASUVORK5CYII=) no-repeat center;background-size:12px;width:12px;height:12px}', 0), a.insertRule('.iv-ad-info-icon {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAsAAAALCAMAAACecocUAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAVFBMVEVMaXGUlJSYmJiZmZmYmJiXl5eZmZmZmZmWlpaVlZWOjo6ZmZmSkpKXl5eYmJiYmJiZmZmZmZmZmZmZmZmYmJiJiYmXl5eZmZmYmJiWlpaZmZn///+81lbeAAAAGnRSTlMAE5DM80DliTMMEjccWIM5p1UjaTQNgB5cLlr5mgUAAAABYktHRBsCYNSkAAAAVElEQVQI102NRw7AIBADhw7ppIf/PzQLJ/ZgWSNrFlDaWKMVcs6HmGLwTqjEME6CFDrAXBYIGhNh3TJEg02wHydctvFc7sbrvnXZV8/zfs3T+7u/P7CrAso35YfPAAAAAElFTkSuQmCC) no-repeat center;background-size:11px;width:11px;height:11px}', 0), a.insertRule('.annotation-close-button {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQBAMAAADt3eJSAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAALVBMVEVMaXEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA/Pz9aWloBAQGZmZlbW1v///+X9wUzAAAACHRSTlMANprf+g6lyRmB9hUAAAABYktHRA5vvTBPAAAAWUlEQVQI12NgYBAycVZkAIKwDiBIZWBgrQAx2gMY2DrAIIFBomPWju6VHY0MGh1rbu891dHEYNGx9+yd2x3NDB4d3XfO7uhoQTDgUnDFcO1wA+FWwC2FOQMAdKg6tUSAFEAAAAAASUVORK5CYII=) no-repeat center}', 0), a.insertRule('.annotation-link-icon {background:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACIAAAAiCAMAAAANmfvwAAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAAAUVBMVEVMaXH////////////////////////////////////////////////////////////////////////////////////////////////////////JzkR1AAAAGnRSTlMAfXf+c3xsdGdv/GJoXPtXXflSVk5L7DBH9VeFfsQAAAABYktHRAH/Ai3eAAAAgElEQVQ4y93SSQ6AIAwFULSOOOJs739Qf9SF0VA2uNCu+psHaQJK7cVCqY+Rg92PXA++Q84KnCR03UIRJrFEKMEgZYFQhpyzQHSBWJJAdIVUENtJ3SC0mu3EdOh7zXZiBrRdzQLJ0Y1GfOlpVstD3HaZktX9X/gvRCxvxL6FR7IBS1RTM5xIpLoAAAAASUVORK5CYII=) no-repeat center}', 0);
        }, Uob = function (a) {
            a = a.createElement({
                I: 'div',
                Ka: [
                    'annotation',
                    'annotation-type-custom'
                ]
            });
            g.Ms(a, !1);
            return a;
        }, Vob = function (a, b) {
            b = !b.isCued() && !g.mG(b, 1024);
            g.DF(a.B, b);
            g.DF(a.C, b);
        }, Wob = function (a, b, c) {
            a.Z = !0;
            a.G = g.CB(b, c);
        }, Xob = function (a, b) {
            for (var c = {}, d = g.u(b.attributes), e = d.next(); !e.done; e = d.next())
                e = e.value, c[e.name] = e.nodeValue;
            for (d = 0; d < b.childNodes.length; d++)
                if (e = b.childNodes[d], g.bb(e) && 1 == e.nodeType) {
                    if (c[e.tagName])
                        var f = c[e.tagName];
                    else if ('data' === e.tagName) {
                        0 < e.childNodes.length && (f = e.childNodes[0].nodeValue, c[e.tagName] = 'string' === typeof f ? f.trim() : f);
                        continue;
                    } else
                        f = [], c[e.tagName] = f;
                    e && 'TEXT' === e.tagName ? 1 === e.childNodes.length && 3 === e.childNodes[0].nodeType ? f.push(e.childNodes[0].nodeValue) : f.push('') : e && f.push(Xob(a, e));
                }
            return c;
        }, apb = function (a) {
            var b = a.player.getVideoData();
            if (b.Va) {
                var c = a.player.U().Na.get(b.videoId);
                if (c) {
                    var d = {
                        format: 'XML',
                        urlParams: {},
                        method: 'POST',
                        withCredentials: !0,
                        onFinish: function (e, f, h) {
                            e = a.loadNumber;
                            f = b.videoId;
                            a.loaded && a.loadNumber === e && a.player.getVideoData().videoId === f && (h = g.vB(h) && h.responseXML ? h.responseXML : null) && Yob(a, h);
                        }
                    };
                    g.IC() && (d.onFinish = Zob(a, d.onFinish));
                    d.postParams = { ic_only: '1' };
                    $ob(d, c);
                    a.Z = !0;
                    g.CB(b.Va, d);
                }
            }
        }, $ob = function (a, b) {
            a.method = 'POST';
            a.postParams = a.postParams || {};
            b.FH && (a.postParams.ic_coll = b.FH);
            b.rN && (a.postParams.ic_xml = b.rN);
            b.qL && (a.postParams.ic_track = b.qL);
        }, bpb = function (a) {
            var b = new g.W({ I: 'div' });
            g.Ms(b.element, !1);
            var c = new rob(a.player, b.element, o4(a));
            g.O(c, b);
            b.Ja(a.B.element);
            c.lG();
            return c;
        }, dpb = function (a, b) {
            var c, d;
            if (b = null == (c = b.getWatchNextResponse()) ? void 0 : null == (d = c.cards) ? void 0 : d.cardCollectionRenderer)
                a.qa = !0, cpb(a, b), b.headerText && a.Sg && (c = g.HF(b.headerText), a.Sg.setAttribute('title', c));
        }, o4 = function (a) {
            if (!a.Y) {
                var b = new Lnb(a);
                g.O(a, b);
                var c = new g.Kp(a);
                g.O(a, c);
                a.Y = new Lob(b, c, a.player.U(), a.player.getVideoData(), a.logger, a.player, a.pj);
            }
            return a.Y;
        }, Yob = function (a, b) {
            var c = !1, d = b.getElementsByTagName('annotations');
            if (d && !(1 > d.length) && (d = d[0].getAttribute('itct'))) {
                var e = g.aF();
                if (e) {
                    var f = g.$E();
                    f && g.gB(g.xwa)(void 0, e, f, [g.ZE(d)]);
                }
            }
            b = b.getElementsByTagName('annotation');
            for (d = 0; d < b.length; d++) {
                f = Xob(a, b[d]);
                e = null;
                try {
                    if (f) {
                        var h = f.id, l = /.+/;
                        var m = 'string' === typeof h && null != l && null != h && h.match(l) ? h : '';
                        var n = W3(f.type, epb), p = W3(f.style, fpb), q = aob(f.data), r = 0 !== q.length ? JSON.parse(q) : {};
                        var t = f.action;
                        f = gob;
                        if (null == t)
                            var v = null;
                        else if (g.ab(t)) {
                            l = [];
                            for (var w = g.u(t), A = w.next(); !A.done; A = w.next()) {
                                var C = f(A.value);
                                C && l.push(C);
                            }
                            v = l;
                        } else {
                            var F = f(t);
                            v = F ? [F] : [];
                        }
                        e = m && n ? new hob(m, n, p, v, r) : null;
                    } else
                        e = null;
                } catch (ma) {
                }
                if (e)
                    if ('branding' === e.type || 'promotion' === e.type) {
                        f = a;
                        l = e;
                        var G = Uob(f), L = null;
                        switch (l.type) {
                        case 'branding':
                            if (f.player.U().Ud)
                                break;
                            f.B.element.appendChild(G);
                            L = new m4(G, o4(f), l);
                            break;
                        case 'promotion':
                            g.cU(f.player, G, 4), L = new Mob(G, o4(f), l);
                        }
                        L && L.lG();
                        if (f = L)
                            g.O(a, f), a.N[e.id] = f;
                    } else if ('card' === e.type || 'drawer' === e.type) {
                        a.j || (a.j = bpb(a), g.O(a, a.j));
                        if ('card' === e.type) {
                            G = a.j;
                            var M = (c = e) && c.data && c.data.card_type;
                            e = c.data;
                            if (M)
                                switch (f = e.tracking || {}, f = {
                                        cY: f.impression,
                                        click: f.click,
                                        close: f.close,
                                        K0: f.teaser_impression,
                                        lL: f.teaser_click
                                    }, l = e.tracking_params || {}, L = null, M) {
                                case 'collaborator':
                                    c = {
                                        id: c.id,
                                        timestamp: e.timestamp || 0,
                                        type: e.card_type,
                                        teaserText: e.teaser_text,
                                        teaserDurationMs: e.teaser_duration_ms,
                                        startMs: e.start_ms,
                                        autoOpen: e.auto_open || !1,
                                        sessionData: e.session_data || {},
                                        sponsored: e.sponsored || !1,
                                        aj: f,
                                        Zo: l.card ? g.ZE(l.card) : null,
                                        Gj: l.teaser ? g.ZE(l.teaser) : null,
                                        Hf: l.icon ? g.ZE(l.icon) : null,
                                        channelId: e.channel_id,
                                        customMessage: e.custom_message ? e.custom_message : null,
                                        profileImageUrl: e.image_url,
                                        title: e.title,
                                        metaInfo: e.meta_info,
                                        url: Y3({
                                            pause_on_navigation: e.pause_on_navigation,
                                            target: e.target || 'new',
                                            value: e.url
                                        }),
                                        onClickCommand: null
                                    };
                                    g4(G, c);
                                    break;
                                case 'playlist':
                                    c = {
                                        id: c.id,
                                        timestamp: e.timestamp || 0,
                                        type: e.card_type,
                                        teaserText: e.teaser_text,
                                        teaserDurationMs: e.teaser_duration_ms,
                                        startMs: e.start_ms,
                                        autoOpen: e.auto_open || !1,
                                        sessionData: e.session_data || {},
                                        sponsored: e.sponsored || !1,
                                        aj: f,
                                        Zo: l.card ? g.ZE(l.card) : null,
                                        Gj: l.teaser ? g.ZE(l.teaser) : null,
                                        Hf: l.icon ? g.ZE(l.icon) : null,
                                        CG: e.image_url,
                                        playlistVideoCount: e.playlist_video_count,
                                        customMessage: e.custom_message ? e.custom_message : null,
                                        title: e.title,
                                        metaInfo: e.meta_info,
                                        url: Y3({
                                            pause_on_navigation: e.pause_on_navigation,
                                            target: e.target || 'new',
                                            value: e.url
                                        }),
                                        onClickCommand: null
                                    };
                                    g4(G, c);
                                    break;
                                case 'productListing':
                                    e.signin_url && (L = Y3({
                                        target: 'current',
                                        value: e.signin_url
                                    }));
                                    M = [];
                                    for (var R = e.offers || [], ea = 0; ea < R.length; ea++)
                                        M.push(new nob(g.Ye(R[ea].merchant), g.Ye(R[ea].price)));
                                    c = {
                                        id: c.id,
                                        timestamp: e.timestamp || 0,
                                        type: e.card_type,
                                        teaserText: e.teaser_text,
                                        teaserDurationMs: e.teaser_duration_ms,
                                        startMs: e.start_ms,
                                        autoOpen: e.auto_open || !1,
                                        sessionData: e.session_data || {},
                                        sponsored: e.sponsored || !1,
                                        aj: f,
                                        Zo: l.card ? g.ZE(l.card) : null,
                                        Gj: l.teaser ? g.ZE(l.teaser) : null,
                                        Hf: l.icon ? g.ZE(l.icon) : null,
                                        imageUrl: e.image_url,
                                        displayDomain: e.display_domain ? e.display_domain : null,
                                        showLinkIcon: !!e.show_link_icon,
                                        YC: e.button_icon_url ? e.button_icon_url : null,
                                        title: e.title,
                                        customMessage: e.custom_message ? e.custom_message : null,
                                        url: Y3({
                                            pause_on_navigation: e.pause_on_navigation,
                                            target: e.target || 'new',
                                            value: e.url
                                        }),
                                        Wpb: L,
                                        Vpb: e.signin_title ? e.signin_title : void 0,
                                        Upb: e.signin_message ? e.signin_message : void 0,
                                        offers: M,
                                        onClickCommand: null
                                    };
                                    g4(G, c);
                                    break;
                                case 'simple':
                                    c = {
                                        id: c.id,
                                        timestamp: e.timestamp || 0,
                                        type: e.card_type,
                                        teaserText: e.teaser_text,
                                        teaserDurationMs: e.teaser_duration_ms,
                                        startMs: e.start_ms,
                                        autoOpen: e.auto_open || !1,
                                        sessionData: e.session_data || {},
                                        sponsored: e.sponsored || !1,
                                        aj: f,
                                        Zo: l.card ? g.ZE(l.card) : null,
                                        Gj: l.teaser ? g.ZE(l.teaser) : null,
                                        Hf: l.icon ? g.ZE(l.icon) : null,
                                        imageUrl: e.image_url,
                                        displayDomain: e.display_domain ? e.display_domain : null,
                                        showLinkIcon: !!e.show_link_icon,
                                        YC: e.button_icon_url ? e.button_icon_url : null,
                                        title: e.title,
                                        customMessage: e.custom_message ? e.custom_message : null,
                                        url: Y3({
                                            pause_on_navigation: e.pause_on_navigation,
                                            target: e.target || 'new',
                                            value: e.url
                                        }),
                                        onClickCommand: null
                                    };
                                    g4(G, c);
                                    break;
                                case 'video':
                                    c = {
                                        id: c.id,
                                        timestamp: e.timestamp || 0,
                                        type: e.card_type,
                                        teaserText: e.teaser_text,
                                        teaserDurationMs: e.teaser_duration_ms,
                                        startMs: e.start_ms,
                                        autoOpen: e.auto_open || !1,
                                        sessionData: e.session_data || {},
                                        sponsored: e.sponsored || !1,
                                        aj: f,
                                        Zo: l.card ? g.ZE(l.card) : null,
                                        Gj: l.teaser ? g.ZE(l.teaser) : null,
                                        Hf: l.icon ? g.ZE(l.icon) : null,
                                        CG: e.image_url,
                                        videoDuration: e.video_duration || null,
                                        customMessage: e.custom_message ? e.custom_message : null,
                                        title: e.title,
                                        metaInfo: e.meta_info,
                                        isLiveNow: !!e.is_live_now,
                                        url: Y3({
                                            pause_on_navigation: e.pause_on_navigation,
                                            target: e.target || 'new',
                                            value: e.url
                                        }),
                                        onClickCommand: null
                                    }, g4(G, c);
                                }
                        } else
                            sob(a.j, e);
                        c = !0;
                    }
            }
            c && (H3(a.player), a.cK());
        }, cpb = function (a, b) {
            var c = !1;
            a.j || (a.j = bpb(a), g.O(a, a.j));
            for (var d = g.u(b.cards || []), e = d.next(); !e.done; e = d.next())
                e = e.value, e.cardRenderer && (tob(a.j, e.cardRenderer), c = !0);
            if (c) {
                var f;
                null != (f = a.player.getVideoData()) && g.rT(f) || (c = a.j, d = b.headerText ? g.HF(b.headerText) : '', g.Of(c.Hb, d), c.Z && c.Z.setAttribute('title', d), c.context.videoData.eventId && (c.eventId = c.context.videoData.eventId), c.Da = b.trackingParams ? g.ZE(b.trackingParams) : null, c.N = b.closeButton.infoCardIconRenderer.trackingParams ? g.ZE(b.closeButton.infoCardIconRenderer.trackingParams) : null, c.Hf = b.icon.infoCardIconRenderer.trackingParams ? g.ZE(b.icon.infoCardIconRenderer.trackingParams) : null, a.cK());
                H3(a.player);
            }
        }, gpb = function (a, b, c, d, e) {
            if (!a.player.U().Ud) {
                var f = [];
                b.navigationEndpoint && g.V(b.navigationEndpoint, g.kT) && g.V(b.navigationEndpoint, g.kT).browseId && f.push(new dob('openUrl', 'click', new bob('/channel/' + g.V(b.navigationEndpoint, g.kT).browseId, 'new', !0, !0)));
                var h = b.watermark.thumbnails[0];
                d = {
                    channel_name: b.channelName,
                    end_ms: b.endTimeMs,
                    image_height: h.height,
                    image_type: 1,
                    image_url: h.url,
                    image_width: h.width,
                    is_mobile: !1,
                    session_data: {
                        annotation_id: c,
                        ei: e,
                        feature: 'iv',
                        itct: b.trackingParams,
                        src_vid: d
                    },
                    start_ms: b.startTimeMs
                };
                if (b.subscribeButton && g.V(b.subscribeButton, g.lT)) {
                    d.channel_id = g.V(b.subscribeButton, g.lT).channelId;
                    var l;
                    b = g.V(b.subscribeButton, g.lT);
                    h = e = null;
                    b.subscribed ? (b.subscriberCountWithUnsubscribeText && (e = g.HF(b.subscriberCountWithUnsubscribeText)), b.subscriberCountText && (h = g.HF(b.subscriberCountText))) : (b.subscriberCountText && (e = g.HF(b.subscriberCountText)), b.subscriberCountWithSubscribeText && (h = g.HF(b.subscriberCountWithSubscribeText)));
                    var m, n = (null == (m = g.V(null == (l = b.signInEndpoint) ? void 0 : l.commandMetadata, g.A2)) ? void 0 : m.url) || '';
                    l = {
                        subscribeText: g.HF(b.unsubscribedButtonText),
                        subscribeCount: e || '',
                        unsubscribeText: g.HF(b.subscribedButtonText),
                        unsubscribeCount: h || '',
                        enabled: b.enabled || !1,
                        classic: !1,
                        subscribed: b.subscribed || !1,
                        feature: 'iv',
                        signInUrl: n
                    };
                    d.standalone_subscribe_button_data = l;
                }
                f = new hob(c, 'branding', 'branding', f, d);
                l = Uob(a);
                a.B.element.appendChild(l);
                f = new m4(l, o4(a), f);
                f.lG();
                g.O(f, f);
                a.N[c] = f;
            }
        }, Zob = function (a, b) {
            return function () {
                var c = g.Ia.apply(0, arguments);
                a.Ma() || a.ra.push(g.Zu.Mi(function () {
                    b.apply(null, g.qa(c));
                }));
            };
        }, hpb = function (a) {
            return 'annotation-editor' === a || 'live-dashboard' === a;
        };
    g.VT.prototype.rB = g.fa(28, function (a, b) {
        var c = g.HV(this.yb());
        c && c.rB(a, b);
    });
    var J3 = {}, K3 = null;
    g.lb(M3, g.zv);
    g.k = M3.prototype;
    g.k.getDuration = function () {
        return this.duration;
    };
    g.k.play = function (a) {
        if (a || 0 == this.j)
            this.progress = 0, this.coords = this.B;
        else if (this.isPlaying())
            return !1;
        L3(this);
        this.startTime = a = g.kb();
        this.isPaused() && (this.startTime -= this.duration * this.progress);
        this.endTime = this.startTime + this.duration;
        this.D = this.startTime;
        this.progress || this.TJ();
        this.un('play');
        this.isPaused() && this.un('resume');
        this.j = 1;
        var b = g.fb(this);
        b in J3 || (J3[b] = this);
        Enb();
        Fnb(this, a);
        return !0;
    };
    g.k.stop = function (a) {
        L3(this);
        this.j = 0;
        a && (this.progress = 1);
        Gnb(this, this.progress);
        this.onStop();
        this.vr();
    };
    g.k.pause = function () {
        this.isPlaying() && (L3(this), this.j = -1, this.un('pause'));
    };
    g.k.va = function () {
        0 == this.j || this.stop(!1);
        this.un('destroy');
        M3.Of.va.call(this);
    };
    g.k.destroy = function () {
        this.dispose();
    };
    g.k.TL = function () {
        this.un('animate');
    };
    g.k.un = function (a) {
        this.dispatchEvent(new Hnb(a, this));
    };
    g.lb(Hnb, g.zb);
    g.lb(N3, M3);
    N3.prototype.C = function () {
    };
    N3.prototype.TL = function () {
        this.C();
        N3.Of.TL.call(this);
    };
    N3.prototype.vr = function () {
        this.C();
        N3.Of.vr.call(this);
    };
    N3.prototype.TJ = function () {
        this.C();
        N3.Of.TJ.call(this);
    };
    g.lb(Inb, N3);
    Inb.prototype.C = function () {
        this.element.style.left = Math.round(this.coords[0]) + 'px';
        this.element.style.top = Math.round(this.coords[1]) + 'px';
    };
    g.y(Lnb, g.I);
    g.k = Lnb.prototype;
    g.k.listen = function (a, b, c, d) {
        c = (0, g.gb)(c, d || this.B);
        a = g.jC(a, b, c);
        this.j.push(a);
        return a;
    };
    g.k.uJ = function (a, b, c, d) {
        c = (0, g.gb)(c, d || this.B);
        a = Cnb(a, b, c);
        this.j.push(a);
        return a;
    };
    g.k.Pc = function (a) {
        g.kC(a);
        g.Hb(this.j, a);
    };
    g.k.removeAll = function () {
        g.kC(this.j);
        this.j.length = 0;
    };
    g.k.va = function () {
        this.removeAll();
        g.I.prototype.va.call(this);
    };
    g.y(Onb, g.qV);
    g.k = Onb.prototype;
    g.k.load = function () {
        g.qV.prototype.load.call(this);
        if (!Q3(this)) {
            var a = g.DUa(this.player.getVideoData());
            a ? (a = Nnb(a, Pnb(this)), R3(this, a, !1)) : Qnb(this);
        }
    };
    g.k.unload = function () {
        R3(this, null);
        this.C && (this.C.abort(), this.C = null);
        g.qV.prototype.unload.call(this);
    };
    g.k.Oh = function (a, b) {
        return Q3(this) ? 'loadCustomEndscreenRenderer' === a ? (a = Nnb(b, 'new'), R3(this, a), !0) : null : null;
    };
    g.k.getOptions = function () {
        return Q3(this) ? ['loadCustomEndscreenRenderer'] : [];
    };
    g.k.Tb = function () {
        if (this.endscreen && this.endscreen.elements) {
            var a = this.player.getVideoContentRect();
            if (a && 0 !== a.width && 0 !== a.height) {
                var b = this.player.getPlayerSize();
                if (b && 0 !== b.width && 0 !== b.height) {
                    var c = a.width / a.height;
                    var d = 0;
                    for (var e = -1, f = 0; f < ipb.length; f++) {
                        var h = Math.abs(b.width - ipb[f]);
                        if (-1 === e || d >= h)
                            e = f, d = h;
                    }
                    d = jpb[e];
                    this.B && g.zs(this.B.element, 'outline-width', Math.max(b.width, b.height) + 'px');
                    for (b = 0; b < this.endscreen.elements.length; ++b)
                        if (f = this.endscreen.elements[b].id, e = this.j[f], h = this.G[f], e && h) {
                            var l = h.width * c / h.aspectRatio, m = Math.round(h.width * a.width);
                            f = Math.round(l * a.height);
                            var n = a.left + Math.round(h.left * a.width), p = a.top + Math.round(h.top * a.height);
                            g.Ks(e.element, m, f);
                            g.Gs(e.element, n, p);
                            g.xv(e.element, kpb);
                            256 < m || 256 < f ? g.uv(e.element, 'ytp-ce-large-round') : 96 < m || 96 < f ? g.uv(e.element, 'ytp-ce-medium-round') : g.uv(e.element, 'ytp-ce-small-round');
                            g.xv(e.element, lpb);
                            m = h.left + h.width / 2;
                            h = h.top + l / 2;
                            g.uv(e.element, 0.5 >= m && 0.5 >= h ? 'ytp-ce-top-left-quad' : 0.5 < m && 0.5 >= h ? 'ytp-ce-top-right-quad' : 0.5 >= m && 0.5 < h ? 'ytp-ce-bottom-left-quad' : 'ytp-ce-bottom-right-quad');
                            g.xv(e.element, jpb);
                            g.uv(e.element, d);
                            (e = g.mf(document, 'div', 'ytp-ce-expanding-overlay-body', e.element)[0]) && g.zs(e, 'height', f + 'px');
                        }
                }
            }
        }
    };
    g.k.onCueRangeEnter = function (a) {
        if (this.endscreen)
            if ('ytp-ce-in-endscreen' === a.getId())
                V3(this, this.endscreen.impressionUrls), (a = g.aF()) && this.endscreen.visualElement && g.qF(a, this.endscreen.visualElement);
            else {
                a = a.getId().substring(15);
                var b = this.j[a], c = this.G[a];
                g.uv(b.element, 'ytp-ce-element-show');
                b.element.removeAttribute('aria-hidden');
                b = this.player.getRootNode();
                g.uv(b, 'ytp-ce-shown');
                V3(this, c.impressionUrls);
                (b = g.aF()) && g.qF(b, c.visualElement);
                this.player.U().N && this.player.gb('endscreenelementshown', a);
            }
    };
    g.k.onCueRangeExit = function (a) {
        if ('ytp-ce-in-endscreen' !== a.getId()) {
            a = a.getId().substring(15);
            var b = this.j[a];
            g.wv(b.element, 'ytp-ce-element-show');
            b.element.setAttribute('aria-hidden', 'true');
            b = this.player.getRootNode();
            g.wv(b, 'ytp-ce-shown');
            this.player.U().N && this.player.gb('endscreenelementhidden', a);
        }
    };
    g.k.zba = function (a) {
        var b = this;
        a.target === window && new g.lv(function () {
            for (var c = g.u(Object.values(b.j)), d = c.next(); !d.done; d = c.next())
                g.xv(d.value.element, [
                    'ytp-ce-force-expand',
                    'ytp-ce-element-hover',
                    'ytp-ce-element-shadow-show'
                ]);
        }, 0).start();
    };
    var ipb = [
            346,
            426,
            470,
            506,
            570,
            640,
            853,
            1280,
            1920
        ], jpb = 'ytp-ce-size-346 ytp-ce-size-426 ytp-ce-size-470 ytp-ce-size-506 ytp-ce-size-570 ytp-ce-size-640 ytp-ce-size-853 ytp-ce-size-1280 ytp-ce-size-1920'.split(' '), lpb = [
            'ytp-ce-top-left-quad',
            'ytp-ce-top-right-quad',
            'ytp-ce-bottom-left-quad',
            'ytp-ce-bottom-right-quad'
        ], kpb = [
            'ytp-ce-small-round',
            'ytp-ce-medium-round',
            'ytp-ce-large-round'
        ];
    var cob = {
        iva: 'current',
        oWa: 'new'
    };
    var eob = {
            CLOSE: 'close',
            f_a: 'openUrl',
            SUBSCRIBE: 'subscribe'
        }, fob = {
            rta: 'click',
            CLOSE: 'close',
            jHa: 'hidden',
            j5a: 'rollOut',
            k5a: 'rollOver',
            b7a: 'shown'
        };
    hob.prototype.Oe = function () {
        var a = iob(this, function (b) {
            return 'openUrl' === b.type && null != b.url;
        });
        return a ? a.url : null;
    };
    var fpb = {
            Kna: 'anchored',
            l2: 'branding',
            CHANNEL: 'channel',
            Yua: 'cta',
            mHa: 'highlightText',
            qLa: 'label',
            PLAYLIST: 'playlist',
            POPUP: 'popup',
            c$a: 'speech',
            SUBSCRIBE: 'subscribe',
            Scb: 'title',
            VIDEO: 'video',
            vkb: 'website'
        }, epb = {
            l2: 'branding',
            Tqa: 'card',
            jxa: 'drawer',
            lHa: 'highlight',
            GRa: 'marker',
            u3a: 'promotion',
            TEXT: 'text',
            bmb: 'widget'
        };
    g.y(b4, g.I);
    g.k = b4.prototype;
    g.k.addCueRange = function (a, b, c, d, e) {
        a = new g.DJ(a, b, {
            id: c,
            namespace: 'annotations_module'
        });
        d && this.Wa.set(a, d);
        e && this.Za.set(a, e);
        this.context.J.lf([a]);
    };
    g.k.lG = function () {
        this.context.B.subscribe('resize', this.zH, this);
    };
    g.k.Ib = function () {
        return this.element;
    };
    g.k.UL = function (a, b, c, d, e, f) {
        if (this.rb)
            return !1;
        f && (f.stopPropagation(), f.preventDefault());
        this.navigate(a, c, d, e);
        return !1;
    };
    g.k.show = function () {
    };
    g.k.hide = function () {
    };
    g.k.destroy = function () {
        g.Ff(this.Ib());
    };
    g.k.zH = function () {
    };
    g.k.navigate = function (a, b, c, d) {
        var e = this, f = X3(a);
        if (f) {
            var h = pob(f, a.target), l = function () {
                    a.j && e.context.J.pauseVideo();
                    var m = e.context.videoData.Kf || !1, n = g.lB(f || '');
                    m && n && (n.v || n.list) ? e.context.J.Io(n.v, b, n.list, !1) : g.NG(f || '', 'current' === h ? '_top' : void 0, b);
                };
            'new' === h && (l(), l = null);
            n4(this.context.logger, c, l, d);
            oob(f) || (c = g.aF(), d = b.itct, c && d && g.tF(c, g.ZE(d)));
        }
    };
    g.k.va = function () {
        this.Wa.clear();
        this.Za.clear();
        g.I.prototype.va.call(this);
    };
    g.k.createElement = function (a) {
        a = new g.W(a);
        g.O(this, a);
        return a.element;
    };
    g.y(rob, b4);
    g.k = rob.prototype;
    g.k.jt = function () {
        this.ra && g4(this, this.ra);
    };
    g.k.isAvailable = function () {
        var a;
        if (a = !!this.cards.length)
            (a = this.J.getRootNode()) ? (a = g.Ls(a), a = 173 < a.width && 173 < a.height) : a = !1;
        return a;
    };
    g.k.zH = function () {
        var a = this.isAvailable();
        g.Ms(this.Ib(), a);
        g.yv(this.context.J.getRootNode(), g.MY.IV_DRAWER_ENABLED, a);
        H3(this.J);
    };
    g.k.destroy = function () {
        this.J.rB(!1);
        try {
            this.J.getRootNode().removeChild(this.D);
        } catch (a) {
        }
        g.iE(this.qa);
        g.pC(this.Ra);
        this.Va && this.Va.dispose();
        this.G && this.G.dispose();
        b4.prototype.destroy.call(this);
    };
    g.k.findLastIndex = function (a) {
        if (0 === this.cards.length)
            return 0;
        var b = g.Eb(this.cards, function (c) {
            return a.Cf.startMs > c.Cf.startMs || a.Cf.startMs === c.Cf.startMs && a.Cf.timestamp >= c.Cf.timestamp ? !0 : !1;
        });
        return -1 === b ? 0 : b + 1;
    };
    g.k.q6 = function () {
        if (this.C) {
            n4(this.context.logger, k4(this).aj.close);
            var a = g.aF();
            a && this.N && g.tF(a, this.N);
            f4(this);
        }
    };
    g.k.N6 = function () {
        g.yv(this.D, 'iv-drawer-scrolled', 0 < this.B.scrollTop);
    };
    g.k.k9 = function () {
        var a = g.aF(), b = k4(this);
        b = b ? b.Hf : this.Hf;
        a && b && g.rF(a, [b]);
    };
    g.k.j9 = function () {
        var a = g.aF(), b = k4(this);
        b = b ? b.Hf : this.Hf;
        a && b && g.sF(a, [b]);
    };
    g.k.n3 = function () {
        var a = k4(this);
        n4(this.context.logger, a.aj.K0);
        var b = g.aF();
        if (b && a)
            if (this.J.L('web_infocards_teaser_show_logging_fix')) {
                var c = [];
                a.Gj && c.push(a.Gj);
                a.Hf && c.push(a.Hf);
                0 < c.length && g.rF(b, c);
            } else
                g.rF(b, [
                    a.Gj,
                    a.Hf
                ]);
    };
    g.k.l9 = function () {
        var a = g.aF(), b = k4(this);
        a && b && g.sF(a, [b.Gj]);
    };
    g.k.m3 = function (a) {
        var b = k4(this), c = g.aF();
        this.j ? a ? (a = this.context.logger, n4(a, b.aj.lL), a.J.sendVideoStatsEngageEvent(4, void 0), c && b.Gj && g.tF(c, b.Gj)) : (a = this.context.logger, n4(a, b.aj.lL), a.J.sendVideoStatsEngageEvent(4, void 0), c && b.Hf && g.tF(c, b.Hf)) : (a = this.context.logger, n4(a, b.aj.lL), a.J.sendVideoStatsEngageEvent(4, void 0), c && this.Hf && g.tF(c, this.Hf));
    };
    g.y(l4, b4);
    l4.prototype.lG = function () {
        b4.prototype.lG.call(this);
        Gob(this);
    };
    l4.prototype.show = function () {
        b4.prototype.show.call(this);
        var a = g.aF(), b = this.annotation.data;
        a && b && (b = b.session_data) && g.rF(a, [g.ZE(b.itct)]);
    };
    l4.prototype.hide = function () {
        b4.prototype.hide.call(this);
        var a = g.aF(), b = this.annotation.data;
        a && b && (b = b.session_data) && g.sF(a, [g.ZE(b.itct)]);
    };
    g.y(m4, l4);
    m4.prototype.qx = function () {
        g.uv(this.Ib(), 'iv-branding');
        var a = this.annotation.data;
        this.B = this.createElement({
            I: 'img',
            Ka: [
                'branding-img',
                'iv-click-target'
            ],
            X: {
                'aria-label': 'Channel watermark',
                src: a.image_url,
                width: a.image_width,
                height: a.image_height
            }
        });
        g.Ms(this.B, !1);
        var b = this.createElement({
            I: 'button',
            Ka: [
                'branding-img-container',
                'ytp-button'
            ]
        });
        b.appendChild(this.B);
        this.Ib().appendChild(b);
        var c = this.annotation.Oe();
        c && c4(this, b, c, this.annotation.id, a.session_data);
        Kob(this, a);
    };
    m4.prototype.show = function () {
        if (!this.isActive && (l4.prototype.show.call(this), this.N || (this.qx(), this.N = !0), g.Ms(this.Ib(), !0), this.isActive = !0, this.B)) {
            try {
                Hob(this, this.B);
            } catch (a) {
            }
            g.uv(this.context.J.getRootNode(), 'ytp-branding-shown');
        }
    };
    m4.prototype.hide = function () {
        this.isActive && (l4.prototype.hide.call(this), g.Ms(this.Ib(), !1), this.isActive = !1, g.wv(this.context.J.getRootNode(), 'ytp-branding-shown'));
    };
    m4.prototype.destroy = function () {
        this.j && (this.j.dispose(), this.j = null);
        l4.prototype.destroy.call(this);
    };
    g.y(Mob, l4);
    g.k = Mob.prototype;
    g.k.qx = function () {
        var a = this, b = this.annotation.data;
        if ('cta' === this.annotation.style)
            var c = 6;
        else if ('video' === this.annotation.style || 'playlist' === this.annotation.style)
            c = 7;
        this.K = b.collapsedelay_ms || this.K;
        var d = [
            'iv-promo',
            'iv-promo-inactive'
        ];
        this.Ib().setAttribute('aria-hidden', 'true');
        this.Ib().setAttribute('aria-label', 'Promotion');
        this.Ib().tabIndex = 0;
        var e = this.annotation.Oe(), f = b.image_url;
        if (f) {
            var h = this.createElement({
                I: 'div',
                Ka: [
                    'iv-promo-img',
                    'iv-click-target'
                ]
            });
            f = this.createElement({
                I: 'img',
                X: {
                    src: f,
                    'aria-hidden': 'true'
                }
            });
            h.appendChild(f);
            b.video_duration && !b.is_live ? (f = this.createElement({
                I: 'span',
                S: 'iv-promo-video-duration',
                xa: b.video_duration
            }), h.appendChild(f)) : b.playlist_length && (f = this.createElement({
                I: 'span',
                S: 'iv-promo-playlist-length',
                xa: b.playlist_length.toString()
            }), h.appendChild(f));
            e && c4(this, h, e, this.annotation.id, b.session_data, void 0, c);
        }
        e ? (f = this.createElement({
            I: 'a',
            S: 'iv-promo-txt'
        }), g.Ke(f, X3(e)), this.j = f) : this.j = this.createElement({
            I: 'div',
            S: 'iv-promo-txt'
        });
        switch (this.annotation.style) {
        case 'cta':
        case 'website':
            var l = this.createElement({
                I: 'p',
                V: [{
                        I: 'strong',
                        xa: b.text_line_1
                    }]
            });
            var m = this.createElement({
                I: 'p',
                V: [{
                        I: 'span',
                        S: 'iv-promo-link',
                        xa: b.text_line_2
                    }]
            });
            if (f = b.text_line_3) {
                d.push('iv-promo-website-card-cta-redesign');
                var n = this.createElement({
                    I: 'button',
                    Ka: [
                        'iv-promo-round-expand-icon',
                        'ytp-button'
                    ]
                });
                f = this.createElement({
                    I: 'button',
                    Ka: [
                        'iv-button',
                        'iv-promo-button'
                    ],
                    V: [{
                            I: 'span',
                            S: 'iv-button-content',
                            xa: f
                        }]
                });
                var p = this.createElement({
                    I: 'div',
                    S: 'iv-promo-button-container'
                });
                p.appendChild(f);
                e && c4(this, this.Ib(), e, this.annotation.id, b.session_data, void 0, c);
            }
            g.uv(this.j, 'iv-click-target');
            e && c4(this, this.j, e, this.annotation.id, b.session_data, void 0, c);
            break;
        case 'playlist':
        case 'video':
            l = this.createElement({
                I: 'p',
                V: [{
                        I: 'span',
                        xa: b.text_line_1
                    }]
            }), m = this.createElement({
                I: 'p',
                V: [{
                        I: 'strong',
                        xa: b.text_line_2
                    }]
            }), b.is_live && (l = m, m = this.createElement({
                I: 'span',
                Ka: [
                    'yt-badge',
                    'iv-promo-badge-live'
                ],
                xa: 'LIVE NOW'
            })), g.uv(this.j, 'iv-click-target'), e && c4(this, this.j, e, this.annotation.id, b.session_data, void 0, c), d.push('iv-promo-video');
        }
        l && this.j.appendChild(l);
        m && this.j.appendChild(m);
        this.N.appendChild(this.j);
        p && this.N.appendChild(p);
        c = this.createElement({
            I: 'div',
            S: 'iv-promo-actions'
        });
        this.B = this.createElement({
            I: 'button',
            Ka: [
                'iv-promo-expand',
                'ytp-button'
            ]
        });
        this.B.title = 'Expand';
        this.context.j.listen(this.B, 'click', function (q) {
            Pob(a, 5000, q);
        });
        c.appendChild(this.B);
        g.Ms(this.B, !1);
        this.context.j.listen(this.Ib(), 'mouseover', this.f8, this);
        this.context.j.listen(this.Ib(), 'mouseout', this.e8, this);
        this.context.j.listen(this.Ib(), 'touchend', function (q) {
            Pob(a, 5000, q);
        });
        this.C = this.createElement({
            I: 'button',
            Ka: [
                'iv-promo-close',
                'ytp-button'
            ]
        });
        this.C.title = 'Close';
        this.context.j.listen(this.C, 'click', 'cta' === this.annotation.style && b.text_line_3 ? this.a8 : this.Z7, this);
        c.appendChild(this.C);
        g.vv(this.Ib(), d);
        h && (g.Af(this.Ib(), h), n && h.appendChild(n));
        g.Af(this.Ib(), this.N);
        g.Af(this.Ib(), c);
    };
    g.k.show = function () {
        this.isActive || (l4.prototype.show.call(this), this.Y || (this.qx(), this.Y = !0), g.Ms(this.Ib(), !0), g.Bg(function () {
            g.wv(this.Ib(), 'iv-promo-inactive');
        }, 100, this), this.Ib().removeAttribute('aria-hidden'), this.isActive = !0, Rob(this), Nob(this), Oob(this, this.K));
    };
    g.k.hide = function () {
        this.isActive && (g.uv(this.Ib(), 'iv-promo-inactive'), this.isActive = !1, this.Ib().setAttribute('aria-hidden', 'true'));
    };
    g.k.UL = function (a, b, c, d, e, f) {
        return this.isCollapsed ? !1 : l4.prototype.UL.call(this, a, b, c, d, e, f);
    };
    g.k.f8 = function (a) {
        this.Z = !0;
        Pob(this, 500, a);
    };
    g.k.e8 = function () {
        this.Z = !1;
        Qob(this);
    };
    g.k.Z7 = function (a) {
        a.stopPropagation();
        this.hide();
    };
    g.k.a8 = function (a) {
        a.stopPropagation();
        Rob(this);
        this.isCollapsed = !0;
        g.uv(this.Ib(), 'iv-promo-collapsed-no-delay');
        this.G.start();
    };
    g.k.destroy = function () {
        this.G.dispose();
        l4.prototype.destroy.call(this);
    };
    g.y(Tob, g.qV);
    g.k = Tob.prototype;
    g.k.Oh = function (a, b) {
        if (!hpb(this.player.U().playerStyle))
            return null;
        switch (a) {
        case 'loadCustomAnnotationsXml':
            return (a = g.f2(b)) && Yob(this, a), !0;
        case 'removeCustomAnnotationById':
            return b && this.j && (vob(this.j, b), H3(this.player)), !0;
        }
        return null;
    };
    g.k.getOptions = function () {
        return hpb(this.player.U().playerStyle) ? [
            'loadCustomAnnotationsXml',
            'removeCustomAnnotationById'
        ] : [];
    };
    g.k.ju = function () {
        var a = this.player.U(), b = this.player.getVideoData(), c = a.annotationsLoadPolicy || b.annotationsLoadPolicy;
        return g.ET(b) || g.OT(this.player.app) ? !1 : 1 === c && !b.BW || a.Na.get(b.videoId) || g.sT(b) || g.EUa(b) ? !0 : !1;
    };
    g.k.cK = function () {
        if (this.C) {
            var a = this.player.kb().getVideoContentRect(!0);
            g.Ks(this.C.element, a.width, a.height);
            g.Gs(this.C.element, a.left, a.top);
        }
        if (this.j) {
            var b = this.player.Dn();
            a = this.j;
            b = b.width;
            g.yv(a.D, 'iv-drawer-small', 426 >= b);
            g.yv(a.D, 'iv-drawer-big', 1280 <= b);
        }
    };
    g.k.o3 = function (a) {
        Vob(this, a.state);
        g.mG(a.state, 2) && (this.Hk() && this.Wo() && 2 !== this.player.getPresentingPlayerType() && this.sB(!1), this.rB(!1));
    };
    g.k.load = function () {
        function a(h) {
            var l = b.loadNumber;
            b.G = null;
            b.loaded && b.loadNumber === l && b.player.getVideoData().videoId === d && (h = g.vB(h) && h.responseXML ? h.responseXML : null) && (Yob(b, h), g.uv(b.player.getRootNode(), 'iv-module-loaded'));
        }
        var b = this;
        g.qV.prototype.load.call(this);
        Vob(this, this.player.Qb());
        this.loadNumber++;
        var c = this.player.getVideoData(), d = c.videoId;
        g.IC() && (a = Zob(this, a));
        var e = {
            format: 'XML',
            onFinish: a,
            onError: function () {
                b.G = null;
            },
            urlParams: {}
        };
        c.isPharma && (e.urlParams.pharma = '1');
        e.method = 'POST';
        e.withCredentials = !0;
        var f = this.player.U().Na.get(d);
        f && $ob(e, f);
        f = f && (f.rN || f.FH);
        if (!c.Ku || f)
            c.Va ? Wob(this, c.Va, e) : (this.K = function () {
                if (!b.Z)
                    b.onVideoDataChange(e);
                var h = b.player.getVideoData();
                (null == h ? 0 : g.rT(h)) && !b.qa && dpb(b, h);
            }, this.player.addEventListener('videodatachange', this.K));
        g.cU(this.player, this.C.element, 4);
        this.cK();
        (f = g.sT(c)) && cpb(this, f);
        (f = g.EUa(c)) && f.featuredChannel && gpb(this, f.featuredChannel, f.annotationId || 'branding', c.videoId || null, c.eventId || null);
        this.Sg = g.tf('ytp-cards-button', this.player.getRootNode());
        g.rT(c) && dpb(this, c);
    };
    g.k.onVideoDataChange = function (a) {
        var b = this.player.getVideoData();
        b.Va && Wob(this, b.Va, a);
    };
    g.k.unload = function () {
        this.player.sf('annotations_module');
        for (var a = g.u(Object.keys(this.N)), b = a.next(); !b.done; b = a.next())
            this.N[b.value].destroy();
        this.Y = null;
        this.j && (this.j.destroy(), this.j = null, H3(this.player));
        this.Z = !1;
        this.G && (this.G.abort(), this.G = null);
        this.qa = !1;
        this.N = {};
        this.B.hide();
        g.qV.prototype.unload.call(this);
        this.C.detach();
        this.K && (this.player.removeEventListener('videodatachange', this.K), this.K = null);
    };
    g.k.a_ = function (a) {
        a === this.player.getVideoData().videoId && (this.loaded ? apb(this) : this.load());
    };
    g.k.Hk = function () {
        var a;
        return (null == (a = this.j) ? void 0 : a.isAvailable()) || this.qa;
    };
    g.k.Wo = function () {
        return !!this.j && this.j.C;
    };
    g.k.sB = function (a, b, c) {
        b = void 0 === b ? !1 : b;
        this.Hk();
        this.j && (a ? c ? e4(this.j, c, b) : e4(this.j, 'YOUTUBE_DRAWER_AUTO_OPEN', b) : f4(this.j));
    };
    g.k.rB = function (a, b) {
        this.player.publish(a ? 'cardsteasershow' : 'cardsteaserhide', b);
    };
    g.k.va = function () {
        this.player.U().Na.unsubscribe('vast_info_card_add', this.a_, this);
        g.wv(this.player.getRootNode(), g.MY.IV_DRAWER_OPEN);
        for (var a = this.ra, b = g.Zu, c = 0, d = a.length; c < d; c++)
            b.Mj(a[c]);
        this.ra.length = 0;
        g.qV.prototype.va.call(this);
    };
    g.k.createElement = function (a) {
        a = new g.W(a);
        g.O(this, a);
        return a.element;
    };
    g.pV('annotations_module', Tob);
    g.pV('creatorendscreen', Onb);
}(_yt_player));